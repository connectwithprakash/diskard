use diskard_core::cleaner::{self, DeleteMode};
use diskard_core::config::{Config, IgnoreConfig, RecognizerConfig};
use diskard_core::error::Result;
use diskard_core::finding::{Category, Finding, RiskLevel};
use diskard_core::recognizer::Recognizer;
use diskard_core::recognizers::all_recognizers;
use diskard_core::scanner::{self, ScanOptions, SortOrder};
use diskard_core::size;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_finding(
    path: PathBuf,
    category: Category,
    risk: RiskLevel,
    size: u64,
    age: Option<Duration>,
) -> Finding {
    let last_modified = age.map(|d| SystemTime::now() - d);
    Finding {
        path,
        category,
        risk,
        size_bytes: size,
        description: format!("test finding ({category})"),
        last_modified,
    }
}

/// A fake recognizer that returns canned findings.
struct FakeRecognizer {
    name: &'static str,
    id: &'static str,
    category: Category,
    findings: Vec<Finding>,
}

impl Recognizer for FakeRecognizer {
    fn name(&self) -> &'static str {
        self.name
    }
    fn id(&self) -> &'static str {
        self.id
    }
    fn category(&self) -> Category {
        self.category
    }
    fn scan(&self) -> Result<Vec<Finding>> {
        Ok(self.findings.clone())
    }
}

fn fake_recognizers(findings: Vec<(Category, Vec<Finding>)>) -> Vec<Box<dyn Recognizer>> {
    findings
        .into_iter()
        .enumerate()
        .map(|(i, (cat, f))| {
            // Leak a string so we get 'static — acceptable in tests.
            let id: &'static str = Box::leak(format!("fake-{i}").into_boxed_str());
            let name: &'static str = Box::leak(format!("Fake {i}").into_boxed_str());
            Box::new(FakeRecognizer {
                name,
                id,
                category: cat,
                findings: f,
            }) as Box<dyn Recognizer>
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Recognizer registry tests
// ---------------------------------------------------------------------------

#[test]
fn test_all_recognizers_exist() {
    let recognizers = all_recognizers();
    assert!(recognizers.len() >= 18, "Expected at least 18 recognizers");
}

#[test]
fn test_recognizer_ids_unique() {
    let recognizers = all_recognizers();
    let mut ids: Vec<&str> = recognizers.iter().map(|r| r.id()).collect();
    let original_len = ids.len();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), original_len, "Recognizer IDs must be unique");
}

#[test]
fn test_recognizer_names_nonempty() {
    for r in all_recognizers() {
        assert!(!r.name().is_empty(), "Recognizer name must not be empty");
        assert!(!r.id().is_empty(), "Recognizer id must not be empty");
    }
}

// ---------------------------------------------------------------------------
// Scanner tests (with fake recognizers)
// ---------------------------------------------------------------------------

#[test]
fn test_scanner_runs_without_panic() {
    let recognizers = all_recognizers();
    let config = Config::default();
    let options = ScanOptions::default();
    let result = scanner::scan(&recognizers, &config, &options);
    assert!(result.scan_duration.as_nanos() > 0);
}

#[test]
fn test_scanner_respects_risk_filter() {
    let findings = vec![
        make_finding("/tmp/a".into(), Category::Node, RiskLevel::Safe, 100, None),
        make_finding(
            "/tmp/b".into(),
            Category::Node,
            RiskLevel::Moderate,
            200,
            None,
        ),
        make_finding("/tmp/c".into(), Category::Node, RiskLevel::Risky, 300, None),
    ];
    let recs = fake_recognizers(vec![(Category::Node, findings)]);
    let config = Config::default();

    // Filter: safe only
    let opts = ScanOptions {
        max_risk: RiskLevel::Safe,
        ..Default::default()
    };
    let result = scanner::scan(&recs, &config, &opts);
    assert_eq!(result.findings.len(), 1);
    assert_eq!(result.findings[0].risk, RiskLevel::Safe);

    // Filter: up to moderate
    let opts = ScanOptions {
        max_risk: RiskLevel::Moderate,
        ..Default::default()
    };
    let result = scanner::scan(&recs, &config, &opts);
    assert_eq!(result.findings.len(), 2);
    for f in &result.findings {
        assert!(f.risk <= RiskLevel::Moderate);
    }
}

#[test]
fn test_scanner_min_size_filter() {
    let findings = vec![
        make_finding(
            "/tmp/small".into(),
            Category::Generic,
            RiskLevel::Safe,
            50,
            None,
        ),
        make_finding(
            "/tmp/big".into(),
            Category::Generic,
            RiskLevel::Safe,
            5000,
            None,
        ),
    ];
    let recs = fake_recognizers(vec![(Category::Generic, findings)]);
    let config = Config::default();
    let opts = ScanOptions {
        min_size: 1000,
        ..Default::default()
    };
    let result = scanner::scan(&recs, &config, &opts);
    assert_eq!(result.findings.len(), 1);
    assert_eq!(result.findings[0].size_bytes, 5000);
}

#[test]
fn test_scanner_category_filter() {
    let node_findings = vec![make_finding(
        "/tmp/node".into(),
        Category::Node,
        RiskLevel::Safe,
        100,
        None,
    )];
    let xcode_findings = vec![make_finding(
        "/tmp/xcode".into(),
        Category::Xcode,
        RiskLevel::Safe,
        200,
        None,
    )];
    let recs = fake_recognizers(vec![
        (Category::Node, node_findings),
        (Category::Xcode, xcode_findings),
    ]);
    let config = Config::default();
    let opts = ScanOptions {
        category: Some(Category::Xcode),
        ..Default::default()
    };
    let result = scanner::scan(&recs, &config, &opts);
    assert_eq!(result.findings.len(), 1);
    assert_eq!(result.findings[0].category, Category::Xcode);
}

#[test]
fn test_scanner_older_than_filter() {
    let old = Duration::from_secs(60 * 60 * 24 * 30); // 30 days
    let recent = Duration::from_secs(60); // 1 minute
    let findings = vec![
        make_finding(
            "/tmp/old".into(),
            Category::Generic,
            RiskLevel::Safe,
            100,
            Some(old),
        ),
        make_finding(
            "/tmp/new".into(),
            Category::Generic,
            RiskLevel::Safe,
            200,
            Some(recent),
        ),
    ];
    let recs = fake_recognizers(vec![(Category::Generic, findings)]);
    let config = Config::default();
    let opts = ScanOptions {
        older_than: Some(Duration::from_secs(60 * 60 * 24)), // 1 day
        ..Default::default()
    };
    let result = scanner::scan(&recs, &config, &opts);
    assert_eq!(result.findings.len(), 1);
    assert!(result.findings[0].path.to_str().unwrap().contains("old"));
}

#[test]
fn test_scanner_sort_by_size() {
    let findings = vec![
        make_finding(
            "/tmp/small".into(),
            Category::Generic,
            RiskLevel::Safe,
            10,
            None,
        ),
        make_finding(
            "/tmp/big".into(),
            Category::Generic,
            RiskLevel::Safe,
            9999,
            None,
        ),
        make_finding(
            "/tmp/mid".into(),
            Category::Generic,
            RiskLevel::Safe,
            500,
            None,
        ),
    ];
    let recs = fake_recognizers(vec![(Category::Generic, findings)]);
    let config = Config::default();
    let opts = ScanOptions {
        sort: SortOrder::Size,
        ..Default::default()
    };
    let result = scanner::scan(&recs, &config, &opts);
    let sizes: Vec<u64> = result.findings.iter().map(|f| f.size_bytes).collect();
    assert_eq!(
        sizes,
        vec![9999, 500, 10],
        "Should be sorted descending by size"
    );
}

#[test]
fn test_scanner_sort_by_risk() {
    let findings = vec![
        make_finding(
            "/tmp/safe".into(),
            Category::Generic,
            RiskLevel::Safe,
            100,
            None,
        ),
        make_finding(
            "/tmp/risky".into(),
            Category::Generic,
            RiskLevel::Risky,
            100,
            None,
        ),
        make_finding(
            "/tmp/moderate".into(),
            Category::Generic,
            RiskLevel::Moderate,
            100,
            None,
        ),
    ];
    let recs = fake_recognizers(vec![(Category::Generic, findings)]);
    let config = Config::default();
    let opts = ScanOptions {
        sort: SortOrder::Risk,
        ..Default::default()
    };
    let result = scanner::scan(&recs, &config, &opts);
    let risks: Vec<RiskLevel> = result.findings.iter().map(|f| f.risk).collect();
    assert_eq!(
        risks,
        vec![RiskLevel::Risky, RiskLevel::Moderate, RiskLevel::Safe],
        "Should be sorted descending by risk"
    );
}

#[test]
fn test_scanner_total_reclaimable() {
    let findings = vec![
        make_finding(
            "/tmp/a".into(),
            Category::Generic,
            RiskLevel::Safe,
            100,
            None,
        ),
        make_finding(
            "/tmp/b".into(),
            Category::Generic,
            RiskLevel::Safe,
            250,
            None,
        ),
    ];
    let recs = fake_recognizers(vec![(Category::Generic, findings)]);
    let config = Config::default();
    let result = scanner::scan(&recs, &config, &ScanOptions::default());
    assert_eq!(result.total_reclaimable, 350);
}

#[test]
fn test_scanner_empty_recognizers() {
    let recs: Vec<Box<dyn Recognizer>> = vec![];
    let config = Config::default();
    let result = scanner::scan(&recs, &config, &ScanOptions::default());
    assert!(result.findings.is_empty());
    assert_eq!(result.total_reclaimable, 0);
}

// ---------------------------------------------------------------------------
// Cleaner tests (with temp dirs)
// ---------------------------------------------------------------------------

#[test]
fn test_cleaner_dry_run() {
    let tmp = TempDir::new().unwrap();
    let file_path = tmp.path().join("testfile.txt");
    std::fs::write(&file_path, "hello world").unwrap();

    let findings = vec![Finding {
        path: file_path.clone(),
        category: Category::Generic,
        risk: RiskLevel::Safe,
        size_bytes: 11,
        description: "test".into(),
        last_modified: None,
    }];

    let result = cleaner::clean(&findings, DeleteMode::DryRun).unwrap();
    assert_eq!(result.deleted_count, 1);
    assert_eq!(result.freed_bytes, 11);
    assert!(file_path.exists(), "Dry run should NOT delete the file");
}

#[test]
fn test_cleaner_permanent_delete_file() {
    let tmp = TempDir::new().unwrap();
    let file_path = tmp.path().join("to_delete.txt");
    std::fs::write(&file_path, "goodbye").unwrap();
    assert!(file_path.exists());

    let findings = vec![Finding {
        path: file_path.clone(),
        category: Category::Generic,
        risk: RiskLevel::Safe,
        size_bytes: 7,
        description: "test".into(),
        last_modified: None,
    }];

    let result = cleaner::clean(&findings, DeleteMode::Permanent).unwrap();
    assert_eq!(result.deleted_count, 1);
    assert_eq!(result.freed_bytes, 7);
    assert!(
        !file_path.exists(),
        "Permanent delete should remove the file"
    );
}

#[test]
fn test_cleaner_permanent_delete_directory() {
    let tmp = TempDir::new().unwrap();
    let dir_path = tmp.path().join("subdir");
    std::fs::create_dir(&dir_path).unwrap();
    std::fs::write(dir_path.join("a.txt"), "aaa").unwrap();
    std::fs::write(dir_path.join("b.txt"), "bbb").unwrap();
    assert!(dir_path.exists());

    let findings = vec![Finding {
        path: dir_path.clone(),
        category: Category::Generic,
        risk: RiskLevel::Safe,
        size_bytes: 6,
        description: "test dir".into(),
        last_modified: None,
    }];

    let result = cleaner::clean(&findings, DeleteMode::Permanent).unwrap();
    assert_eq!(result.deleted_count, 1);
    assert!(
        !dir_path.exists(),
        "Permanent delete should remove the directory"
    );
}

#[test]
fn test_cleaner_nonexistent_path_is_ok() {
    let findings = vec![Finding {
        path: PathBuf::from("/nonexistent/path/that/does/not/exist"),
        category: Category::Generic,
        risk: RiskLevel::Safe,
        size_bytes: 999,
        description: "ghost".into(),
        last_modified: None,
    }];

    let result = cleaner::clean(&findings, DeleteMode::Permanent).unwrap();
    assert_eq!(result.deleted_count, 1);
    assert!(result.errors.is_empty());
}

#[test]
fn test_cleaner_empty_findings() {
    let result = cleaner::clean(&[], DeleteMode::Permanent).unwrap();
    assert_eq!(result.deleted_count, 0);
    assert_eq!(result.freed_bytes, 0);
    assert!(result.errors.is_empty());
}

#[test]
fn test_cleaner_multiple_findings() {
    let tmp = TempDir::new().unwrap();
    let f1 = tmp.path().join("f1.txt");
    let f2 = tmp.path().join("f2.txt");
    let f3 = tmp.path().join("f3.txt");
    std::fs::write(&f1, "one").unwrap();
    std::fs::write(&f2, "two").unwrap();
    std::fs::write(&f3, "three").unwrap();

    let findings: Vec<Finding> = [(&f1, 3u64), (&f2, 3), (&f3, 5)]
        .iter()
        .map(|(p, s)| Finding {
            path: p.to_path_buf(),
            category: Category::Generic,
            risk: RiskLevel::Safe,
            size_bytes: *s,
            description: "test".into(),
            last_modified: None,
        })
        .collect();

    let result = cleaner::clean(&findings, DeleteMode::Permanent).unwrap();
    assert_eq!(result.deleted_count, 3);
    assert_eq!(result.freed_bytes, 11);
    assert!(!f1.exists());
    assert!(!f2.exists());
    assert!(!f3.exists());
}

// ---------------------------------------------------------------------------
// Config tests
// ---------------------------------------------------------------------------

#[test]
fn test_config_default() {
    let config = Config::default();
    assert_eq!(config.defaults.delete_mode, "trash");
    assert_eq!(config.defaults.risk_tolerance, "moderate");
    assert_eq!(config.max_risk(), RiskLevel::Moderate);
    assert!(config.is_recognizer_enabled("xcode-derived-data"));
}

#[test]
fn test_config_load_from_file() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("config.toml");
    std::fs::write(
        &config_path,
        r#"
[defaults]
risk_tolerance = "safe"
delete_mode = "permanent"
min_size = 1024

[ignore]
paths = ["/tmp/keep-this"]

[recognizers]
disabled = ["docker-data"]
"#,
    )
    .unwrap();

    let config = Config::load_from(&config_path).unwrap();
    assert_eq!(config.max_risk(), RiskLevel::Safe);
    assert_eq!(config.defaults.delete_mode, "permanent");
    assert_eq!(config.defaults.min_size, 1024);
    assert!(config.is_path_ignored(std::path::Path::new("/tmp/keep-this/subdir")));
    assert!(!config.is_recognizer_enabled("docker-data"));
    assert!(config.is_recognizer_enabled("xcode-derived-data"));
}

#[test]
fn test_config_path_ignore() {
    let config = Config {
        ignore: IgnoreConfig {
            paths: vec![PathBuf::from("/home/user/important")],
        },
        ..Default::default()
    };
    assert!(config.is_path_ignored(std::path::Path::new("/home/user/important")));
    assert!(config.is_path_ignored(std::path::Path::new("/home/user/important/subdir")));
    assert!(!config.is_path_ignored(std::path::Path::new("/home/user/other")));
}

#[test]
fn test_config_disabled_recognizers() {
    let mut disabled = std::collections::HashSet::new();
    disabled.insert("npm-cache".to_string());
    disabled.insert("homebrew-cache".to_string());

    let config = Config {
        recognizers: RecognizerConfig { disabled },
        ..Default::default()
    };
    assert!(!config.is_recognizer_enabled("npm-cache"));
    assert!(!config.is_recognizer_enabled("homebrew-cache"));
    assert!(config.is_recognizer_enabled("xcode-derived-data"));
}

#[test]
fn test_config_risk_tolerance_parsing() {
    let mut config = Config::default();

    config.defaults.risk_tolerance = "safe".to_string();
    assert_eq!(config.max_risk(), RiskLevel::Safe);

    config.defaults.risk_tolerance = "moderate".to_string();
    assert_eq!(config.max_risk(), RiskLevel::Moderate);

    config.defaults.risk_tolerance = "risky".to_string();
    assert_eq!(config.max_risk(), RiskLevel::Risky);

    // Unknown defaults to moderate
    config.defaults.risk_tolerance = "unknown".to_string();
    assert_eq!(config.max_risk(), RiskLevel::Moderate);

    // Case-insensitive
    config.defaults.risk_tolerance = "SAFE".to_string();
    assert_eq!(config.max_risk(), RiskLevel::Safe);
}

#[test]
fn test_scanner_respects_ignored_paths() {
    let findings = vec![
        make_finding(
            "/home/user/important/cache".into(),
            Category::Generic,
            RiskLevel::Safe,
            100,
            None,
        ),
        make_finding(
            "/tmp/deletable".into(),
            Category::Generic,
            RiskLevel::Safe,
            200,
            None,
        ),
    ];
    let recs = fake_recognizers(vec![(Category::Generic, findings)]);
    let config = Config {
        ignore: IgnoreConfig {
            paths: vec![PathBuf::from("/home/user/important")],
        },
        ..Default::default()
    };
    let result = scanner::scan(&recs, &config, &ScanOptions::default());
    assert_eq!(result.findings.len(), 1);
    assert!(result.findings[0]
        .path
        .to_str()
        .unwrap()
        .contains("deletable"));
}

#[test]
fn test_scanner_respects_disabled_recognizers() {
    let findings = vec![make_finding(
        "/tmp/x".into(),
        Category::Generic,
        RiskLevel::Safe,
        100,
        None,
    )];
    let recs = fake_recognizers(vec![(Category::Generic, findings)]);

    // Disable the fake recognizer
    let mut disabled = std::collections::HashSet::new();
    disabled.insert("fake-0".to_string());
    let config = Config {
        recognizers: RecognizerConfig { disabled },
        ..Default::default()
    };
    let result = scanner::scan(&recs, &config, &ScanOptions::default());
    assert!(result.findings.is_empty());
}

// ---------------------------------------------------------------------------
// Size utility tests
// ---------------------------------------------------------------------------

#[test]
fn test_size_formatting() {
    assert_eq!(size::format_bytes(0), "0 B");
    assert_eq!(size::format_bytes(1024), "1.0 kiB");
    assert_eq!(size::format_bytes(1_073_741_824), "1.0 GiB");
}

#[test]
fn test_dir_size_with_fixture() {
    let tmp = TempDir::new().unwrap();
    // Create files with known sizes
    std::fs::write(tmp.path().join("a.txt"), vec![0u8; 1000]).unwrap();
    std::fs::write(tmp.path().join("b.txt"), vec![0u8; 2000]).unwrap();
    let sub = tmp.path().join("sub");
    std::fs::create_dir(&sub).unwrap();
    std::fs::write(sub.join("c.txt"), vec![0u8; 500]).unwrap();

    let total = size::dir_size(tmp.path());
    assert_eq!(total, 3500, "Dir size should sum all files recursively");
}

#[test]
fn test_dir_size_empty_dir() {
    let tmp = TempDir::new().unwrap();
    assert_eq!(size::dir_size(tmp.path()), 0);
}

#[test]
fn test_dir_size_single_file() {
    let tmp = TempDir::new().unwrap();
    let file_path = tmp.path().join("single.bin");
    std::fs::write(&file_path, vec![0u8; 4096]).unwrap();
    assert_eq!(size::dir_size(&file_path), 4096);
}

#[test]
fn test_dir_size_nonexistent() {
    assert_eq!(size::dir_size(std::path::Path::new("/no/such/path")), 0);
}

// ---------------------------------------------------------------------------
// Finding tests
// ---------------------------------------------------------------------------

#[test]
fn test_finding_size_human() {
    let finding = Finding {
        path: PathBuf::from("/tmp/test"),
        category: Category::Generic,
        risk: RiskLevel::Safe,
        size_bytes: 1_073_741_824,
        description: "Test".into(),
        last_modified: None,
    };
    assert_eq!(finding.size_human(), "1.0 GiB");
}

#[test]
fn test_risk_level_ordering() {
    assert!(RiskLevel::Safe < RiskLevel::Moderate);
    assert!(RiskLevel::Moderate < RiskLevel::Risky);
    assert!(RiskLevel::Safe < RiskLevel::Risky);
}

#[test]
fn test_risk_level_display() {
    assert_eq!(format!("{}", RiskLevel::Safe), "safe");
    assert_eq!(format!("{}", RiskLevel::Moderate), "moderate");
    assert_eq!(format!("{}", RiskLevel::Risky), "risky");
}

#[test]
fn test_category_display() {
    assert_eq!(format!("{}", Category::Xcode), "Xcode");
    assert_eq!(format!("{}", Category::Node), "Node.js");
    assert_eq!(format!("{}", Category::HuggingFace), "HuggingFace");
    assert_eq!(format!("{}", Category::VSCode), "VS Code");
    assert_eq!(format!("{}", Category::CocoaPods), "CocoaPods");
}

#[test]
fn test_finding_serializable() {
    let finding = Finding {
        path: PathBuf::from("/tmp/test"),
        category: Category::Xcode,
        risk: RiskLevel::Moderate,
        size_bytes: 1024,
        description: "Test finding".into(),
        last_modified: Some(SystemTime::now()),
    };
    let json = serde_json::to_string(&finding).unwrap();
    assert!(json.contains("\"category\":\"Xcode\""));
    assert!(json.contains("\"risk\":\"Moderate\""));
    assert!(json.contains("\"size_bytes\":1024"));
    // last_modified is #[serde(skip)]
    assert!(!json.contains("last_modified"));
}
