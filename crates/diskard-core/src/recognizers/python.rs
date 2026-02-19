use crate::error::Result;
use crate::finding::{Category, Finding, RiskLevel};
use crate::recognizer::Recognizer;
use crate::size::dir_size;

/// pip download cache.
pub struct PipCache;

impl Recognizer for PipCache {
    fn name(&self) -> &'static str {
        "pip cache"
    }

    fn id(&self) -> &'static str {
        "pip-cache"
    }

    fn category(&self) -> Category {
        Category::Python
    }

    fn scan(&self) -> Result<Vec<Finding>> {
        let Some(home) = dirs::home_dir() else {
            return Ok(vec![]);
        };
        let path = home.join("Library/Caches/pip");
        if !path.exists() {
            return Ok(vec![]);
        }

        let size = dir_size(&path);
        if size == 0 {
            return Ok(vec![]);
        }

        Ok(vec![Finding {
            path,
            category: Category::Python,
            risk: RiskLevel::Safe,
            size_bytes: size,
            description: "pip package cache — re-downloaded on next install".into(),
            last_modified: None,
        }])
    }
}
