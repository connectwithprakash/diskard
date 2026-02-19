use crate::error::Result;
use crate::finding::{Category, Finding, RiskLevel};
use crate::recognizer::Recognizer;
use crate::size::dir_size;

/// HuggingFace Hub cache — downloaded models and datasets.
pub struct HuggingFaceCache;

impl Recognizer for HuggingFaceCache {
    fn name(&self) -> &'static str {
        "HuggingFace cache"
    }

    fn id(&self) -> &'static str {
        "huggingface-cache"
    }

    fn category(&self) -> Category {
        Category::HuggingFace
    }

    fn scan(&self) -> Result<Vec<Finding>> {
        let Some(home) = dirs::home_dir() else {
            return Ok(vec![]);
        };
        let path = home.join(".cache/huggingface");
        if !path.exists() {
            return Ok(vec![]);
        }

        let size = dir_size(&path);
        if size == 0 {
            return Ok(vec![]);
        }

        Ok(vec![Finding {
            path,
            category: Category::HuggingFace,
            risk: RiskLevel::Moderate,
            size_bytes: size,
            description: "HuggingFace model and dataset cache — re-downloaded when needed".into(),
            last_modified: None,
        }])
    }
}
