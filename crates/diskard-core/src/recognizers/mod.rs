mod claude;
mod docker;
mod generic;
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
        // Xcode (4 recognizers)
        Box::new(xcode::DerivedData),
        Box::new(xcode::DeviceSupport),
        Box::new(xcode::Simulators),
        Box::new(xcode::Previews),
        // Node.js
        Box::new(node::NpmCache),
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
        // Generic
        Box::new(generic::DsStore),
    ]
}
