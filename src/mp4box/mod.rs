use std::io::SeekFrom;
use async_trait::async_trait;
use futures::{AsyncRead, AsyncSeek, AsyncSeekExt, AsyncWrite};
use crate::error::MalformedBoxError::UnknownSizeForUnknownBox;
use crate::error::MP4Error;
use crate::header::BoxHeader;
use crate::r#type::BoxType;
use crate::size::BoxSize::Known;

pub mod r#box;
pub mod full_box;
pub mod mvhd;
pub mod moov;
pub mod mvex;
pub mod trex;

pub trait IBox {
    fn byte_size(&self) -> usize;
    fn id() -> BoxType;
}

#[async_trait]
pub trait BoxRead<R: AsyncRead + AsyncSeek>: IBox + Sized {
    async fn read(header: BoxHeader, reader: &mut R) -> Result<Self, MP4Error>;
}

#[async_trait]
pub trait BoxWrite<W: AsyncWrite>: IBox {
    async fn write(&self, writer: &mut W) -> Result<usize, MP4Error>;
}

pub trait PartialBox {
    type ParentData;
    fn byte_size(&self) -> usize;
    fn id() -> BoxType;
}

#[async_trait]
pub trait PartialBoxRead<R>: PartialBox + Sized
    where
        R: AsyncRead + AsyncSeek + Unpin + Send + Sync
{
    async fn read_data(parent_data: Self::ParentData, reader: &mut R) -> Result<Self, MP4Error>;
    async fn read_child(&mut self, header: BoxHeader, reader: &mut R) -> Result<(), MP4Error> {
        Ok(())
    }
}

#[async_trait]
pub trait PartialBoxWrite<W>: PartialBox
    where
        W: AsyncWrite + Unpin + Send + Sync
{
    async fn write_data(&self, writer: &mut W) -> Result<usize, MP4Error> {Ok(0)}
    async fn write_children(&self, writer: &mut W) -> Result<usize, MP4Error> {Ok(0)}
}
