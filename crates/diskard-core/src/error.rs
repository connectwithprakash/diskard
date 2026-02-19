use std::path::PathBuf;

/// All errors that can occur in diskard-core.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error at {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Config error: {0}")]
    Config(String),

    #[error("Config parse error: {0}")]
    ConfigParse(#[from] toml::de::Error),

    #[error("Trash error: {0}")]
    Trash(String),

    #[error("Scanner error: {0}")]
    Scanner(String),
}

impl Error {
    pub fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
