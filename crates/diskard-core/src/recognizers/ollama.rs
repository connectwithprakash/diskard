use crate::error::Result;
use crate::finding::{Category, Finding, RiskLevel};
use crate::recognizer::Recognizer;
use crate::size::dir_size;

/// Ollama downloaded models.
pub struct OllamaModels;

impl Recognizer for OllamaModels {
    fn name(&self) -> &'static str {
        "Ollama models"
    }

    fn id(&self) -> &'static str {
        "ollama-models"
    }

    fn category(&self) -> Category {
        Category::Ollama
    }

    fn scan(&self) -> Result<Vec<Finding>> {
        let Some(home) = dirs::home_dir() else {
            return Ok(vec![]);
        };
        let path = home.join(".ollama/models");
        if !path.exists() {
            return Ok(vec![]);
        }

        let size = dir_size(&path);
        if size == 0 {
            return Ok(vec![]);
        }

        Ok(vec![Finding {
            path,
            category: Category::Ollama,
            risk: RiskLevel::Moderate,
            size_bytes: size,
            description: "Ollama model files — re-downloaded with `ollama pull`".into(),
            last_modified: None,
        }])
    }
}
