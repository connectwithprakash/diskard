use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};
use crate::finding::RiskLevel;

/// Top-level configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub defaults: Defaults,
    pub ignore: IgnoreConfig,
    pub recognizers: RecognizerConfig,
}

/// Default behavior settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Defaults {
    /// Maximum risk level to show by default.
    pub risk_tolerance: String,
    /// "trash" or "permanent".
    pub delete_mode: String,
    /// Minimum size in bytes to report.
    pub min_size: u64,
}

/// Paths and patterns to ignore.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct IgnoreConfig {
    /// Absolute paths to never scan or delete.
    pub paths: Vec<PathBuf>,
}

/// Per-recognizer configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RecognizerConfig {
    /// Recognizer IDs to disable.
    pub disabled: HashSet<String>,
}

impl Default for Defaults {
    fn default() -> Self {
        Self {
            risk_tolerance: "moderate".to_string(),
            delete_mode: "trash".to_string(),
            min_size: 0,
        }
    }
}

impl Config {
    /// Standard config file path: `~/.config/diskard/config.toml`
    pub fn path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("diskard").join("config.toml"))
    }

    /// Load config from the default path, falling back to defaults if not found.
    pub fn load() -> Result<Self> {
        match Self::path() {
            Some(path) if path.exists() => Self::load_from(&path),
            _ => Ok(Self::default()),
        }
    }

    /// Load config from a specific path.
    pub fn load_from(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| Error::io(path, e))?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Write default config to the standard path.
    pub fn init() -> Result<PathBuf> {
        let path = Self::path()
            .ok_or_else(|| Error::Config("Cannot determine config directory".into()))?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| Error::io(parent, e))?;
        }
        let content =
            toml::to_string_pretty(&Config::default()).map_err(|e| Error::Config(e.to_string()))?;
        std::fs::write(&path, content).map_err(|e| Error::io(&path, e))?;
        Ok(path)
    }

    /// Parse the risk tolerance string into a RiskLevel.
    pub fn max_risk(&self) -> RiskLevel {
        match self.defaults.risk_tolerance.to_lowercase().as_str() {
            "safe" => RiskLevel::Safe,
            "risky" => RiskLevel::Risky,
            _ => RiskLevel::Moderate,
        }
    }

    /// Check if a recognizer is enabled.
    pub fn is_recognizer_enabled(&self, id: &str) -> bool {
        !self.recognizers.disabled.contains(id)
    }

    /// Check if a path is ignored.
    pub fn is_path_ignored(&self, path: &Path) -> bool {
        self.ignore
            .paths
            .iter()
            .any(|ignored| path.starts_with(ignored))
    }
}
