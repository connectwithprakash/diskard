use crate::error::Result;
use crate::finding::{Category, Finding, RiskLevel};
use crate::recognizer::Recognizer;
use crate::size::dir_size;
use std::path::PathBuf;

/// Cargo target directories — build artifacts from Rust projects.
pub struct CargoTarget;

impl Recognizer for CargoTarget {
    fn name(&self) -> &'static str {
        "Cargo target dirs"
    }

    fn id(&self) -> &'static str {
        "cargo-target"
    }

    fn category(&self) -> Category {
        Category::Rust
    }

    fn scan(&self) -> Result<Vec<Finding>> {
        let Some(home) = dirs::home_dir() else {
            return Ok(vec![]);
        };

        // Also check the cargo registry cache
        let mut findings = Vec::new();

        let registry_cache = home.join(".cargo/registry/cache");
        if registry_cache.exists() {
            let size = dir_size(&registry_cache);
            if size > 0 {
                findings.push(Finding {
                    path: registry_cache,
                    category: Category::Rust,
                    risk: RiskLevel::Safe,
                    size_bytes: size,
                    description: "Cargo registry cache — re-downloaded when needed".into(),
                    last_modified: None,
                });
            }
        }

        // Scan common developer directories for Cargo projects
        let scan_roots: Vec<PathBuf> = vec![
            home.join("Developer"),
            home.join("Projects"),
            home.join("src"),
        ];

        for root in scan_roots {
            if !root.exists() {
                continue;
            }
            self.scan_for_targets(&root, &mut findings);
        }

        Ok(findings)
    }
}

impl CargoTarget {
    fn scan_for_targets(&self, root: &PathBuf, findings: &mut Vec<Finding>) {
        // Walk top 3 levels looking for Cargo.toml + target/
        let walker = jwalk::WalkDir::new(root)
            .max_depth(4)
            .skip_hidden(true)
            .into_iter();

        for entry in walker.flatten() {
            if entry.file_name().to_str() == Some("Cargo.toml") && entry.file_type().is_file() {
                if let Some(parent) = entry.path().parent() {
                    let target = parent.join("target");
                    if target.exists() && target.is_dir() {
                        let size = dir_size(&target);
                        if size > 1_048_576 {
                            // Only report if > 1MB
                            findings.push(Finding {
                                path: target,
                                category: Category::Rust,
                                risk: RiskLevel::Moderate,
                                size_bytes: size,
                                description: format!(
                                    "Rust build artifacts for {}",
                                    parent.file_name().unwrap_or_default().to_string_lossy()
                                ),
                                last_modified: None,
                            });
                        }
                    }
                }
            }
        }
    }
}
