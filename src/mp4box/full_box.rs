
use std::ops::Deref;
use futures::{AsyncRead, AsyncSeek, AsyncWrite};
use crate::error::MP4Error;
use crate::header::BoxHeader;
use crate::mp4box::{PartialBox, PartialBoxRead, PartialBoxWrite};
use async_trait::async_trait;
use crate::bytes_read::ReadMp4;
use crate::bytes_write::WriteMp4;
use crate::r#type::BoxType;

#[derive(Debug, Copy, Clone)]
pub struct FullBoxData {
    pub version: u8,
    pub flags: u32
}

impl FullBoxData {

    pub const fn byte_size() -> usize {
        4
    }

    pub async fn read<R: ReadMp4>(reader: &mut R) -> Result<FullBoxData, MP4Error> {
        let version = reader.read_u8().await?;
        let flags = reader.read_u24().await?;
        Ok(Self{version, flags})
    }

    pub async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        count += writer.write_u8(self.version).await?;
        count += writer.write_u24(self.flags).await?;
        Ok(count)
    }
}

pub trait FullBoxInfo {
    fn version(&self) -> u8 {0}
    fn flags(&self) -> u32 {0}
}

#[derive(Debug, Clone, Default)]
pub struct FullBox<P>
    where
        P: PartialBox<ParentData=FullBoxData> + FullBoxInfo
{
    pub inner: P,
}

impl<P> From<P> for FullBox<P>
    where
        P: PartialBox<ParentData=FullBoxData> + FullBoxInfo
{
    fn from(inner: P) -> Self {
        Self{inner}
    }
}

impl<P: PartialBox<ParentData=FullBoxData> + FullBoxInfo> PartialBox for FullBox<P> {
    type ParentData = ();

    fn byte_size(&self) -> usize {
        FullBoxData::byte_size() + self.inner.byte_size()
    }

    const ID: BoxType = P::ID;
}

#[async_trait]
impl<P: PartialBox<ParentData=FullBoxData> + PartialBoxRead<R> + FullBoxInfo + Send + Sync, R: AsyncRead + AsyncSeek + Unpin + Send + Sync> PartialBoxRead<R> for FullBox<P> {
    async fn read_data(_: Self::ParentData, reader: &mut R) -> Result<Self, MP4Error> {
        let data = FullBoxData::read(reader).await?;
        let inner = P::read_data(data, reader).await?;
        Ok(Self { inner })
    }

    async fn read_child(&mut self, header: BoxHeader, reader: &mut R) -> Result<(), MP4Error> {
        self.inner.read_child(header, reader).await
    }
}

#[async_trait]
impl<P: PartialBox<ParentData=FullBoxData> + PartialBoxWrite<W> + FullBoxInfo + Send + Sync, W: AsyncWrite + Unpin + Send + Sync> PartialBoxWrite<W> for FullBox<P> {

    async fn write_data(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        let version = self.inner.version();
        let flags = self.inner.flags();
        count += FullBoxData{ version, flags }.write(writer).await?;
        count += self.inner.write_data(writer).await?;
        Ok(count)
    }

    async fn write_children(&self, writer: &mut W) -> Result<usize, MP4Error> {
        self.inner.write_children(writer).await
    }
}

impl<P: PartialBox<ParentData=FullBoxData> + FullBoxInfo> Deref for FullBox<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
