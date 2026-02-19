use std::path::PathBuf;

use crate::error::Result;
use crate::finding::{Category, Finding, RiskLevel};
use crate::recognizer::Recognizer;
use crate::size::dir_size;

/// npm cache directory.
pub struct NpmCache;

impl Recognizer for NpmCache {
    fn name(&self) -> &'static str {
        "npm cache"
    }

    fn id(&self) -> &'static str {
        "npm-cache"
    }

    fn category(&self) -> Category {
        Category::Node
    }

    fn scan(&self) -> Result<Vec<Finding>> {
        let Some(home) = dirs::home_dir() else {
            return Ok(vec![]);
        };
        let path = home.join(".npm");
        if !path.exists() {
            return Ok(vec![]);
        }

        let size = dir_size(&path);
        if size == 0 {
            return Ok(vec![]);
        }

        Ok(vec![Finding {
            path,
            category: Category::Node,
            risk: RiskLevel::Safe,
            size_bytes: size,
            description: "npm package cache — repopulated on next install".into(),
            last_modified: None,
        }])
    }
}

/// node_modules directories in project trees — project-based scanner.
pub struct NodeModules;

impl Recognizer for NodeModules {
    fn name(&self) -> &'static str {
        "node_modules"
    }

    fn id(&self) -> &'static str {
        "node-modules"
    }

    fn category(&self) -> Category {
        Category::Node
    }

    fn scan(&self) -> Result<Vec<Finding>> {
        let Some(home) = dirs::home_dir() else {
            return Ok(vec![]);
        };

        let mut findings = Vec::new();
        let scan_roots: Vec<PathBuf> = vec![
            home.join("Developer"),
            home.join("Projects"),
            home.join("src"),
        ];

        for root in scan_roots {
            if !root.exists() {
                continue;
            }
            self.scan_for_node_modules(&root, &mut findings);
        }

        Ok(findings)
    }
}

impl NodeModules {
    fn scan_for_node_modules(&self, root: &PathBuf, findings: &mut Vec<Finding>) {
        let walker = jwalk::WalkDir::new(root)
            .max_depth(5)
            .skip_hidden(true)
            .into_iter();

        for entry in walker.flatten() {
            if entry.file_name().to_str() == Some("package.json") && entry.file_type().is_file() {
                if let Some(parent) = entry.path().parent() {
                    let nm = parent.join("node_modules");
                    if nm.exists() && nm.is_dir() {
                        let size = dir_size(&nm);
                        if size > 1_048_576 {
                            findings.push(Finding {
                                path: nm,
                                category: Category::Node,
                                risk: RiskLevel::Safe,
                                size_bytes: size,
                                description: format!(
                                    "node_modules for {}",
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
