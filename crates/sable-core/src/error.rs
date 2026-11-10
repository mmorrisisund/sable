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

    /// Errors when decoding images for textures
    #[error("Error decoding image: {0}")]
    ImageDecode(#[from] image::error::ImageError),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Asset(AssetError::from(err))
    }
}
