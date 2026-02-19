use crate::error::Result;
use crate::finding::{Category, Finding, RiskLevel};
use crate::recognizer::Recognizer;
use crate::size::dir_size;

/// CocoaPods download cache.
pub struct CocoaPodsCache;

impl Recognizer for CocoaPodsCache {
    fn name(&self) -> &'static str {
        "CocoaPods cache"
    }

    fn id(&self) -> &'static str {
        "cocoapods-cache"
    }

    fn category(&self) -> Category {
        Category::CocoaPods
    }

    fn scan(&self) -> Result<Vec<Finding>> {
        let Some(home) = dirs::home_dir() else {
            return Ok(vec![]);
        };
        let path = home.join("Library/Caches/CocoaPods");
        if !path.exists() {
            return Ok(vec![]);
        }

        let size = dir_size(&path);
        if size == 0 {
            return Ok(vec![]);
        }

        Ok(vec![Finding {
            path,
            category: Category::CocoaPods,
            risk: RiskLevel::Safe,
            size_bytes: size,
            description: "CocoaPods download cache — re-downloaded on next pod install".into(),
            last_modified: None,
        }])
    }
}
