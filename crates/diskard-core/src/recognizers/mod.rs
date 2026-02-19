mod claude;
mod cocoapods;
mod docker;
mod generic;
mod gradle;
mod homebrew;
mod huggingface;
mod node;
mod ollama;
mod python;
mod rust;
mod vscode;
mod xcode;

use crate::recognizer::Recognizer;

/// Return all built-in recognizers.
pub fn all_recognizers() -> Vec<Box<dyn Recognizer>> {
    vec![
        // Xcode (5 recognizers)
        Box::new(xcode::DerivedData),
        Box::new(xcode::DeviceSupport),
        Box::new(xcode::Simulators),
        Box::new(xcode::Archives),
        Box::new(xcode::Previews),
        // Node.js (2 recognizers)
        Box::new(node::NpmCache),
        Box::new(node::NodeModules),
        // Homebrew
        Box::new(homebrew::HomebrewCache),
        // Python
        Box::new(python::PipCache),
        // Rust
        Box::new(rust::CargoTarget),
        // Docker
        Box::new(docker::DockerData),
        // Ollama
        Box::new(ollama::OllamaModels),
        // HuggingFace
        Box::new(huggingface::HuggingFaceCache),
        // Claude
        Box::new(claude::ClaudeData),
        // VS Code
        Box::new(vscode::VSCodeExtensions),
        // Gradle / Maven
        Box::new(gradle::GradleCache),
        // CocoaPods
        Box::new(cocoapods::CocoaPodsCache),
        // Generic
        Box::new(generic::DsStore),
    ]
}
