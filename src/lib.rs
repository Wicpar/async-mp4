mod id;

use std::pin::Pin;
use std::task::{Context, Poll};
use async_trait::async_trait;
use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use thiserror::Error;
use id::BoxId;

#[derive(Error, Debug)]
pub enum MP4Error {
    #[error("IO Error")]
    IO(#[from] std::io::Error),
    #[error("unknown data store error")]
    Unknown,
}

pub trait Box {
    fn size(&self) -> usize;
}

#[async_trait]
pub trait BoxRead<R: AsyncRead>: Box {
    async fn read(reader: R) -> Result<Self, MP4Error>;
}

#[async_trait]
pub trait BoxWrite<W: AsyncWrite>: Box {
    async fn write(&self, reader: W) -> Result<usize, MP4Error>;
}

pub trait PartialBox {
    fn data_size(&self) -> usize;
    fn children(&self) -> dyn Iterator<Item=&dyn Box>;
    fn acceptable_children() -> &[BoxId];
}

#[async_trait]
pub trait PartialBoxRead<R: AsyncRead>: PartialBox {
    async fn read_data(reader: R) -> Result<Self, MP4Error>;
}

#[async_trait]
pub trait PartialBoxWrite<W: AsyncWrite>: PartialBox {
    async fn write_data(&self, reader: W) -> Result<usize, MP4Error>;
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
