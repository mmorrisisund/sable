use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    /// Asset loading and processing errors
    #[error("Asset error: {0}")]
    Asset(AssetError),
}

/// Asset loading errors
#[derive(Error, Debug)]
pub enum AssetError {
    /// Unable to locate asset
    #[error("Asset not found: {0}")]
    NotFound(String),

    /// File I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
