use rayon::prelude::*;
use std::time::{Duration, Instant};

use crate::config::Config;
use crate::error::Result;
use crate::finding::{Finding, RiskLevel};
use crate::recognizer::Recognizer;

/// Results from a scan operation.
pub struct ScanResult {
    pub findings: Vec<Finding>,
    pub total_reclaimable: u64,
    pub scan_duration: Duration,
    pub errors: Vec<String>,
}

/// Options for controlling scan behavior.
pub struct ScanOptions {
    pub max_risk: RiskLevel,
    pub min_size: u64,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            max_risk: RiskLevel::Risky,
            min_size: 0,
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

    // Filter to enabled recognizers
    let enabled: Vec<&Box<dyn Recognizer>> = recognizers
        .iter()
        .filter(|r| config.is_recognizer_enabled(r.id()))
        .collect();

    // Run recognizers in parallel
    let results: Vec<Result<Vec<Finding>>> = enabled
        .par_iter()
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
    findings.retain(|f| {
        f.risk <= options.max_risk
            && f.size_bytes >= options.min_size
            && !config.is_path_ignored(&f.path)
    });

    // Sort by size descending
    findings.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));

    let total_reclaimable = findings.iter().map(|f| f.size_bytes).sum();

    ScanResult {
        findings,
        total_reclaimable,
        scan_duration: start.elapsed(),
        errors,
    }
}
