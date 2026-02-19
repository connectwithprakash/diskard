use crate::error::Result;
use crate::finding::{Category, Finding, RiskLevel};
use crate::recognizer::Recognizer;
use crate::size::dir_size;

/// Homebrew download cache.
pub struct HomebrewCache;

impl Recognizer for HomebrewCache {
    fn name(&self) -> &'static str {
        "Homebrew cache"
    }

    fn id(&self) -> &'static str {
        "homebrew-cache"
    }

    fn category(&self) -> Category {
        Category::Homebrew
    }

    fn scan(&self) -> Result<Vec<Finding>> {
        let Some(home) = dirs::home_dir() else {
            return Ok(vec![]);
        };
        let path = home.join("Library/Caches/Homebrew");
        if !path.exists() {
            return Ok(vec![]);
        }

        let size = dir_size(&path);
        if size == 0 {
            return Ok(vec![]);
        }

        Ok(vec![Finding {
            path,
            category: Category::Homebrew,
            risk: RiskLevel::Safe,
            size_bytes: size,
            description: "Homebrew download cache — re-downloaded when needed".into(),
            last_modified: None,
        }])
    }
}
