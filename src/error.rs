use thiserror::Error;
use crate::r#type::BoxType;
use crate::id::BoxId;

#[derive(Error, Debug)]
pub enum MalformedBoxError {
    #[error("Trying to read {actual} as {target}")]
    ReadingWrongBox {
        target: BoxType,
        actual: BoxType
    },
    #[error("Malformed {0} Box: unknown version: {1}")]
    UnknownVersion(BoxType, u8),
    #[error("Malformed {0} Box: {1}")]
    Custom(BoxType, String),
    #[error("Unknown mp4box has unknown size that should read to end")]
    UnknownSizeForUnknownBox,
}
#[derive(Error, Debug)]
pub enum MP4Error {
    #[error("IO Error")]
    IO(#[from] std::io::Error),
    #[error("Malformed Box")]
    MalformedBox(#[from] MalformedBoxError),
    #[error("unknown data store error")]
    Unknown,
}
