use std::io::SeekFrom;
use async_trait::async_trait;
use futures::{AsyncRead, AsyncSeek, AsyncSeekExt, AsyncWrite};
use crate::bytes_read::ReadMp4;
use crate::bytes_write::WriteMp4;
use crate::error::MalformedBoxError::UnknownSizeForUnknownBox;
use crate::error::MP4Error;
use crate::header::BoxHeader;
use crate::r#type::BoxType;
use crate::size::BoxSize::Known;

pub mod rootbox;
pub mod full_box;
pub mod mvhd;
pub mod moov;
pub mod mvex;
pub mod trex;
pub mod trak;
pub mod tkhd;

pub trait IBox {
    fn byte_size(&self) -> usize;
    const ID: BoxType;
}

#[async_trait]
pub trait BoxRead<R: ReadMp4>: IBox + Sized {
    async fn read(header: BoxHeader, reader: &mut R) -> Result<Self, MP4Error>;
}

#[async_trait]
pub trait BoxWrite<W: WriteMp4>: IBox {
    async fn write(&self, writer: &mut W) -> Result<usize, MP4Error>;
}

pub trait PartialBox {
    type ParentData;
    fn byte_size(&self) -> usize;
    const ID: BoxType;
}

#[async_trait]
pub trait PartialBoxRead<R: ReadMp4>: PartialBox + Sized {
    async fn read_data(parent_data: Self::ParentData, reader: &mut R) -> Result<Self, MP4Error>;
    async fn read_child(&mut self, header: BoxHeader, reader: &mut R) -> Result<(), MP4Error> {
        Ok(())
    }
}

#[async_trait]
pub trait PartialBoxWrite<W: WriteMp4>: PartialBox {
    async fn write_data(&self, writer: &mut W) -> Result<usize, MP4Error> {Ok(0)}
    async fn write_children(&self, writer: &mut W) -> Result<usize, MP4Error> {Ok(0)}
}
