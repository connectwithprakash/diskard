use std::time::{Duration, Instant, SystemTime};

use crate::config::Config;
use crate::error::Result;
use crate::finding::{Category, Finding, RiskLevel};
use crate::recognizer::Recognizer;

/// Results from a scan operation.
pub struct ScanResult {
    pub findings: Vec<Finding>,
    pub total_reclaimable: u64,
    pub scan_duration: Duration,
    pub errors: Vec<String>,
}

/// How to sort findings.
#[derive(Debug, Clone, Copy, Default)]
pub enum SortOrder {
    #[default]
    Size,
    Risk,
    Category,
}

/// Options for controlling scan behavior.
pub struct ScanOptions {
    pub max_risk: RiskLevel,
    pub min_size: u64,
    pub category: Option<Category>,
    pub older_than: Option<Duration>,
    pub sort: SortOrder,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            max_risk: RiskLevel::Risky,
            min_size: 0,
            category: None,
            older_than: None,
            sort: SortOrder::Size,
        }
    }
}

/// Run all enabled recognizers in parallel and collect findings.
pub fn scan(
    recognizers: &[Box<dyn Recognizer>],
    config: &Config,
    options: &ScanOptions,
) -> ScanResult {
    let start = Instant::now();

    // Filter to enabled recognizers, optionally by category
    let enabled: Vec<&Box<dyn Recognizer>> = recognizers
        .iter()
        .filter(|r| config.is_recognizer_enabled(r.id()))
        .filter(|r| options.category.is_none() || Some(r.category()) == options.category)
        .collect();

    // Run recognizers sequentially to avoid file descriptor exhaustion.
    // Each recognizer uses jwalk (rayon-based) internally for dir_size,
    // and running them all in parallel can exceed the OS open-file limit.
    let results: Vec<Result<Vec<Finding>>> = enabled
        .iter()
        .map(|recognizer| {
            log::debug!("Running recognizer: {}", recognizer.name());
            recognizer.scan()
        })
        .collect();

    let mut findings = Vec::new();
    let mut errors = Vec::new();

    for result in results {
        match result {
            Ok(mut f) => findings.append(&mut f),
            Err(e) => errors.push(e.to_string()),
        }
    }

    // Filter by config
    let now = SystemTime::now();
    findings.retain(|f| {
        if f.risk > options.max_risk || f.size_bytes < options.min_size {
            return false;
        }
        if config.is_path_ignored(&f.path) {
            return false;
        }
        if let Some(max_age) = options.older_than {
            if let Some(modified) = f.last_modified {
                if let Ok(age) = now.duration_since(modified) {
                    if age < max_age {
                        return false;
                    }
                }
            }
            // If no last_modified, include it (we can't determine age)
        }
        true
    });

    // Sort
    match options.sort {
        SortOrder::Size => findings.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes)),
        SortOrder::Risk => findings.sort_by(|a, b| b.risk.cmp(&a.risk)),
        SortOrder::Category => {
            findings.sort_by(|a, b| a.category.to_string().cmp(&b.category.to_string()))
        }
    }

    let total_reclaimable = findings.iter().map(|f| f.size_bytes).sum();

    ScanResult {
        findings,
        total_reclaimable,
        scan_duration: start.elapsed(),
        errors,
    }
}
