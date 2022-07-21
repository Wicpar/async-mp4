use std::string::FromUtf8Error;
use thiserror::Error;
use crate::r#type::BoxType;

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
    #[error("Bad Utf8 String")]
    BadUtf8(#[from] FromUtf8Error),
    #[error("Malformed Box")]
    MalformedBox(#[from] MalformedBoxError),
    #[error("{0}")]
    Custom(String),
    #[error("unknown mp4 error")]
    Unknown,
}
