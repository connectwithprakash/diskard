use diskard_core::config::Config;
use diskard_core::finding::{Category, Finding, RiskLevel};
use diskard_core::recognizers::all_recognizers;
use diskard_core::scanner::{self, ScanOptions};
use diskard_core::size;

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
fn test_scanner_runs_without_panic() {
    let recognizers = all_recognizers();
    let config = Config::default();
    let options = ScanOptions::default();

    // Should not panic even if no paths exist
    let result = scanner::scan(&recognizers, &config, &options);
    assert!(result.scan_duration.as_nanos() > 0);
}

#[test]
fn test_scanner_respects_risk_filter() {
    let recognizers = all_recognizers();
    let config = Config::default();
    let options = ScanOptions {
        max_risk: RiskLevel::Safe,
        ..Default::default()
    };

    let result = scanner::scan(&recognizers, &config, &options);

    for finding in &result.findings {
        assert_eq!(
            finding.risk,
            RiskLevel::Safe,
            "Found non-safe finding when filtering for safe only: {}",
            finding.description
        );
    }
}

#[test]
fn test_config_default() {
    let config = Config::default();
    assert_eq!(config.defaults.delete_mode, "trash");
    assert_eq!(config.max_risk(), RiskLevel::Moderate);
    assert!(config.is_recognizer_enabled("xcode-derived-data"));
}

#[test]
fn test_size_formatting() {
    assert_eq!(size::format_bytes(0), "0 B");
    assert_eq!(size::format_bytes(1024), "1.0 kiB");
}

#[test]
fn test_finding_size_human() {
    let finding = Finding {
        path: std::path::PathBuf::from("/tmp/test"),
        category: Category::Generic,
        risk: RiskLevel::Safe,
        size_bytes: 1_073_741_824,
        description: "Test".into(),
        last_modified: None,
    };
    // bytesize uses decimal MB by default
    assert!(
        !finding.size_human().is_empty(),
        "size_human should return a non-empty string"
    );
}

#[test]
fn test_scanner_category_filter() {
    let recognizers = all_recognizers();
    let config = Config::default();
    let options = ScanOptions {
        category: Some(Category::Xcode),
        ..Default::default()
    };

    let result = scanner::scan(&recognizers, &config, &options);

    for finding in &result.findings {
        assert_eq!(
            finding.category,
            Category::Xcode,
            "Found non-Xcode finding when filtering by Xcode: {}",
            finding.description
        );
    }
}

#[test]
fn test_risk_level_ordering() {
    assert!(RiskLevel::Safe < RiskLevel::Moderate);
    assert!(RiskLevel::Moderate < RiskLevel::Risky);
}
