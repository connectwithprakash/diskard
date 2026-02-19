use crate::error::Result;
use crate::finding::{Category, Finding, RiskLevel};
use crate::recognizer::Recognizer;
use crate::size::dir_size;

/// Docker Desktop data (images, containers, volumes).
pub struct DockerData;

impl Recognizer for DockerData {
    fn name(&self) -> &'static str {
        "Docker data"
    }

    fn id(&self) -> &'static str {
        "docker-data"
    }

    fn category(&self) -> Category {
        Category::Docker
    }

    fn scan(&self) -> Result<Vec<Finding>> {
        let Some(home) = dirs::home_dir() else {
            return Ok(vec![]);
        };
        let path = home.join("Library/Containers/com.docker.docker/Data");
        if !path.exists() {
            return Ok(vec![]);
        }

        let size = dir_size(&path);
        if size == 0 {
            return Ok(vec![]);
        }

        Ok(vec![Finding {
            path,
            category: Category::Docker,
            risk: RiskLevel::Risky,
            size_bytes: size,
            description: "Docker Desktop data — includes images, containers, and volumes".into(),
            last_modified: None,
        }])
    }
}
