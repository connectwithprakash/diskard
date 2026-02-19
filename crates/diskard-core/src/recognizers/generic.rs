use crate::error::Result;
use crate::finding::{Category, Finding, RiskLevel};
use crate::recognizer::Recognizer;

/// .DS_Store files scattered across the filesystem.
pub struct DsStore;

impl Recognizer for DsStore {
    fn name(&self) -> &'static str {
        ".DS_Store files"
    }

    fn id(&self) -> &'static str {
        "ds-store"
    }

    fn category(&self) -> Category {
        Category::Generic
    }

    fn scan(&self) -> Result<Vec<Finding>> {
        let Some(home) = dirs::home_dir() else {
            return Ok(vec![]);
        };

        let mut total_size: u64 = 0;
        let mut count: u64 = 0;

        // Walk common developer directories for .DS_Store files
        let scan_roots = vec![
            home.join("Developer"),
            home.join("Projects"),
            home.join("Documents"),
            home.join("Desktop"),
        ];

        for root in scan_roots {
            if !root.exists() {
                continue;
            }

            let walker = jwalk::WalkDir::new(&root)
                .skip_hidden(false)
                .max_depth(6)
                .into_iter();

            for entry in walker.flatten() {
                if entry.file_name().to_str() == Some(".DS_Store") && entry.file_type().is_file() {
                    total_size += entry.metadata().map(|m| m.len()).unwrap_or(0);
                    count += 1;
                }
            }
        }

        if count == 0 {
            return Ok(vec![]);
        }

        Ok(vec![Finding {
            path: home.join("**/.DS_Store"),
            category: Category::Generic,
            risk: RiskLevel::Safe,
            size_bytes: total_size,
            description: format!("{count} .DS_Store files — macOS folder metadata, safe to delete"),
            last_modified: None,
        }])
    }
}
