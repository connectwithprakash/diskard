use crate::error::Result;
use crate::finding::{Category, Finding, RiskLevel};
use crate::recognizer::Recognizer;
use crate::size::dir_size;
use std::path::PathBuf;

fn home() -> Option<PathBuf> {
    dirs::home_dir()
}

/// Xcode DerivedData — build artifacts that regenerate on next build.
pub struct DerivedData;

impl Recognizer for DerivedData {
    fn name(&self) -> &'static str {
        "Xcode DerivedData"
    }

    fn id(&self) -> &'static str {
        "xcode-derived-data"
    }

    fn category(&self) -> Category {
        Category::Xcode
    }

    fn scan(&self) -> Result<Vec<Finding>> {
        let Some(home) = home() else {
            return Ok(vec![]);
        };
        let path = home.join("Library/Developer/Xcode/DerivedData");
        if !path.exists() {
            return Ok(vec![]);
        }

        let size = dir_size(&path);
        if size == 0 {
            return Ok(vec![]);
        }

        Ok(vec![Finding {
            path,
            category: Category::Xcode,
            risk: RiskLevel::Safe,
            size_bytes: size,
            description: "Xcode build artifacts — regenerated on next build".into(),
            last_modified: None,
        }])
    }
}

/// Xcode iOS DeviceSupport — debug symbols for connected devices.
pub struct DeviceSupport;

impl Recognizer for DeviceSupport {
    fn name(&self) -> &'static str {
        "Xcode DeviceSupport"
    }

    fn id(&self) -> &'static str {
        "xcode-device-support"
    }

    fn category(&self) -> Category {
        Category::Xcode
    }

    fn scan(&self) -> Result<Vec<Finding>> {
        let Some(home) = home() else {
            return Ok(vec![]);
        };
        let path = home.join("Library/Developer/Xcode/iOS DeviceSupport");
        if !path.exists() {
            return Ok(vec![]);
        }

        let size = dir_size(&path);
        if size == 0 {
            return Ok(vec![]);
        }

        Ok(vec![Finding {
            path,
            category: Category::Xcode,
            risk: RiskLevel::Moderate,
            size_bytes: size,
            description: "Debug symbols for connected iOS devices — re-downloaded when needed"
                .into(),
            last_modified: None,
        }])
    }
}

/// Xcode Simulators — CoreSimulator device data.
pub struct Simulators;

impl Recognizer for Simulators {
    fn name(&self) -> &'static str {
        "Xcode Simulators"
    }

    fn id(&self) -> &'static str {
        "xcode-simulators"
    }

    fn category(&self) -> Category {
        Category::Xcode
    }

    fn scan(&self) -> Result<Vec<Finding>> {
        let Some(home) = home() else {
            return Ok(vec![]);
        };
        let path = home.join("Library/Developer/CoreSimulator/Devices");
        if !path.exists() {
            return Ok(vec![]);
        }

        let size = dir_size(&path);
        if size == 0 {
            return Ok(vec![]);
        }

        Ok(vec![Finding {
            path,
            category: Category::Xcode,
            risk: RiskLevel::Risky,
            size_bytes: size,
            description: "iOS Simulator device data — deleting removes all simulator content"
                .into(),
            last_modified: None,
        }])
    }
}

/// Xcode SwiftUI Previews cache.
pub struct Previews;

impl Recognizer for Previews {
    fn name(&self) -> &'static str {
        "Xcode Previews"
    }

    fn id(&self) -> &'static str {
        "xcode-previews"
    }

    fn category(&self) -> Category {
        Category::Xcode
    }

    fn scan(&self) -> Result<Vec<Finding>> {
        let Some(home) = home() else {
            return Ok(vec![]);
        };
        let path = home.join("Library/Developer/Xcode/UserData/Previews");
        if !path.exists() {
            return Ok(vec![]);
        }

        let size = dir_size(&path);
        if size == 0 {
            return Ok(vec![]);
        }

        Ok(vec![Finding {
            path,
            category: Category::Xcode,
            risk: RiskLevel::Safe,
            size_bytes: size,
            description: "SwiftUI preview cache — regenerated automatically".into(),
            last_modified: None,
        }])
    }
}
