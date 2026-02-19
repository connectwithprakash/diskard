use serde::Serialize;
use std::fmt;
use std::path::PathBuf;
use std::time::SystemTime;

/// Risk level for a finding — how safe it is to delete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum RiskLevel {
    /// Safe to delete — caches, build artifacts that regenerate automatically.
    Safe,
    /// Moderate — can be regenerated but may take time or bandwidth.
    Moderate,
    /// Risky — may contain user data or require manual reconfiguration.
    Risky,
}

impl fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Safe => write!(f, "safe"),
            Self::Moderate => write!(f, "moderate"),
            Self::Risky => write!(f, "risky"),
        }
    }
}

impl RiskLevel {
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Safe => "🟢",
            Self::Moderate => "🟡",
            Self::Risky => "🔴",
        }
    }
}

/// Category of a finding — which tool/ecosystem it belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Category {
    Xcode,
    Node,
    Homebrew,
    Python,
    Rust,
    Docker,
    Ollama,
    HuggingFace,
    Claude,
    VSCode,
    Gradle,
    CocoaPods,
    Generic,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Xcode => write!(f, "Xcode"),
            Self::Node => write!(f, "Node.js"),
            Self::Homebrew => write!(f, "Homebrew"),
            Self::Python => write!(f, "Python"),
            Self::Rust => write!(f, "Rust"),
            Self::Docker => write!(f, "Docker"),
            Self::Ollama => write!(f, "Ollama"),
            Self::HuggingFace => write!(f, "HuggingFace"),
            Self::Claude => write!(f, "Claude"),
            Self::VSCode => write!(f, "VS Code"),
            Self::Gradle => write!(f, "Gradle"),
            Self::CocoaPods => write!(f, "CocoaPods"),
            Self::Generic => write!(f, "Generic"),
        }
    }
}

/// A single finding — a path that can be cleaned up.
#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    /// Absolute path to the directory or file.
    pub path: PathBuf,
    /// Which ecosystem this belongs to.
    pub category: Category,
    /// How risky it is to delete.
    pub risk: RiskLevel,
    /// Size in bytes.
    pub size_bytes: u64,
    /// Human-readable description of what this is.
    pub description: String,
    /// Last modification time, if available.
    #[serde(skip)]
    pub last_modified: Option<SystemTime>,
}

impl Finding {
    pub fn size_human(&self) -> String {
        crate::size::format_bytes(self.size_bytes)
    }
}
