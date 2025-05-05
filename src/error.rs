use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArchisoError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("Template render error: {0}")]
    Template(#[from] askama::Error),

    #[error("Process failed: {0}")]
    Process(String),
}
