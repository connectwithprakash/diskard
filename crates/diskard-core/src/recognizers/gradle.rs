use crate::error::Result;
use crate::finding::{Category, Finding, RiskLevel};
use crate::recognizer::Recognizer;
use crate::size::dir_size;

/// Gradle build cache and wrapper distributions.
pub struct GradleCache;

impl Recognizer for GradleCache {
    fn name(&self) -> &'static str {
        "Gradle cache"
    }

    fn id(&self) -> &'static str {
        "gradle-cache"
    }

    fn category(&self) -> Category {
        Category::Gradle
    }

    fn scan(&self) -> Result<Vec<Finding>> {
        let Some(home) = dirs::home_dir() else {
            return Ok(vec![]);
        };

        let mut findings = Vec::new();

        // Gradle caches (build outputs, dependency cache)
        let caches = home.join(".gradle/caches");
        if caches.exists() {
            let size = dir_size(&caches);
            if size > 0 {
                findings.push(Finding {
                    path: caches,
                    category: Category::Gradle,
                    risk: RiskLevel::Safe,
                    size_bytes: size,
                    description: "Gradle build and dependency cache — rebuilt on next build".into(),
                    last_modified: None,
                });
            }
        }

        // Gradle wrapper distributions
        let wrapper = home.join(".gradle/wrapper/dists");
        if wrapper.exists() {
            let size = dir_size(&wrapper);
            if size > 0 {
                findings.push(Finding {
                    path: wrapper,
                    category: Category::Gradle,
                    risk: RiskLevel::Moderate,
                    size_bytes: size,
                    description: "Gradle wrapper distributions — re-downloaded when needed".into(),
                    last_modified: None,
                });
            }
        }

        // Maven local repository
        let m2 = home.join(".m2/repository");
        if m2.exists() {
            let size = dir_size(&m2);
            if size > 0 {
                findings.push(Finding {
                    path: m2,
                    category: Category::Gradle,
                    risk: RiskLevel::Moderate,
                    size_bytes: size,
                    description: "Maven local repository — re-downloaded on next build".into(),
                    last_modified: None,
                });
            }
        }

        Ok(findings)
    }
}
