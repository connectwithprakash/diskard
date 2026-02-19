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
