use crate::error::Result;
use crate::finding::{Category, Finding};

/// A recognizer detects reclaimable disk space from a specific tool or ecosystem.
///
/// Recognizers come in two flavors:
/// - **Path-based**: scan known fixed paths (caches, model stores)
/// - **Project-based**: scan directories for marker files (build artifacts)
pub trait Recognizer: Send + Sync {
    /// Human-readable name (e.g., "Xcode DerivedData").
    fn name(&self) -> &'static str;

    /// Machine-readable identifier (e.g., "xcode-derived-data").
    fn id(&self) -> &'static str;

    /// Which category this recognizer belongs to.
    fn category(&self) -> Category;

    /// Scan for findings. Returns an empty vec if nothing found.
    fn scan(&self) -> Result<Vec<Finding>>;
}
