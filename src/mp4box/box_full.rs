
use std::ops::{BitOr, Deref};
use futures::{AsyncRead, AsyncSeek, AsyncWrite};
use crate::error::MP4Error;
use crate::header::BoxHeader;
use async_trait::async_trait;
use crate::bytes_read::ReadMp4;
use crate::bytes_write::{Mp4Writable, WriteMp4};
use crate::mp4box::box_trait::{PartialBox, PartialBoxRead, PartialBoxWrite};
use crate::r#type::BoxType;

#[derive(Debug, Copy, Clone)]
pub struct FullBoxData<F: Default + BitOr + Into<u32> + From<u32>> {
    pub version: u8,
    pub flags: F
}

impl<F: Default + BitOr + Into<u32> + From<u32>> FullBoxData<F> {

    pub const fn byte_size() -> usize {
        4
    }

    pub async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        let version = reader.read().await?;
        let flags = reader.read_u24().await?.into();
        Ok(Self{version, flags})
    }

    pub async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        count += self.version.write(writer).await?;
        count += writer.write_u24(self.flags.into()).await?;
        Ok(count)
    }
}

pub trait FullBoxInfo {
    type Flag: Default + BitOr;
    fn version(&self) -> u8 {0}
    fn flags(&self) -> Self::Flag {Self::Flag::default()}
}

#[derive(Debug, Clone, Default)]
pub struct FullBox<P, F>
    where
        P: PartialBox<ParentData=FullBoxData<F>> + FullBoxInfo,
        F: Default + BitOr + Into<u32> + From<u32>
{
    pub inner: P,
}

impl<P, F> From<P> for FullBox<P, F>
    where
        P: PartialBox<ParentData=FullBoxData<F>> + FullBoxInfo,
        F: Default + BitOr + Into<u32> + From<u32>
{
    fn from(inner: P) -> Self {
        Self{inner}
    }
}

impl<P, F> PartialBox for FullBox<P, F> where
    P: PartialBox<ParentData=FullBoxData<F>> + FullBoxInfo,
    F: Default + BitOr + Into<u32> + From<u32> {
    type ParentData = ();
    type ThisData = FullBoxData<F>;

    fn byte_size(&self) -> usize {
        FullBoxData::byte_size() + self.inner.byte_size()
    }

    const ID: BoxType = P::ID;
}

#[async_trait]
impl<P, F, R> PartialBoxRead<R> for FullBox<P, F> where
    P: PartialBox<ParentData=FullBoxData<F>> + PartialBoxRead<R> + FullBoxInfo + Send + Sync,
    F: Default + BitOr + Into<u32> + From<u32>,
    R: AsyncRead + AsyncSeek + Unpin + Send + Sync {
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
impl<P, F, W> PartialBoxWrite<W> for FullBox<P, F> where
    P: PartialBox<ParentData=FullBoxData<F>> + PartialBoxWrite<W> + FullBoxInfo + Send + Sync,
    F: Default + BitOr + Into<u32> + From<u32>,
    W: AsyncWrite + Unpin + Send + Sync {

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

impl<P, F> Deref for FullBox<P, F>
    where
        P: PartialBox<ParentData=FullBoxData<F>> + FullBoxInfo,
        F: Default + BitOr + Into<u32> + From<u32>,
{
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
