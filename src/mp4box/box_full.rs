use std::ops::{Deref};
use crate::error::MP4Error;
use crate::header::BoxHeader;
use async_trait::async_trait;
use crate::bytes_read::{Mp4Readable, ReadMp4};
use crate::bytes_write::{FlagTrait, Mp4Writable, WriteMp4};
use crate::mp4box::box_trait::{PartialBox, PartialBoxRead, PartialBoxWrite};
use crate::r#type::BoxType;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FullBoxData<F: FlagTrait> {
    pub version: u8,
    pub flags: F
}

#[async_trait]
impl<F: FlagTrait> Mp4Readable for FullBoxData<F> {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        let version = reader.read().await?;
        let flags = reader.read_u24().await?.into();
        Ok(Self{version, flags})
    }
}

#[async_trait]
impl<F: FlagTrait> Mp4Writable for FullBoxData<F> {


    fn byte_size(&self) -> usize {
        4
    }

    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        count += self.version.write(writer).await?;
        count += writer.write_u24(self.flags.into()).await?;
        Ok(count)
    }
}

pub trait FullBoxInfo {
    type Flag: FlagTrait;
    fn version(&self) -> u8 {0}
    fn flags(&self) -> Self::Flag {Self::Flag::default()}
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct FullBox<P, F>
    where
        P: PartialBox<ParentData=FullBoxData<F>> + FullBoxInfo,
        F: FlagTrait
{
    pub inner: P,
}

impl<P, F> From<P> for FullBox<P, F>
    where
        P: PartialBox<ParentData=FullBoxData<F>> + FullBoxInfo,
        F: FlagTrait
{
    fn from(inner: P) -> Self {
        Self{inner}
    }
}

impl<P, F> PartialBox for FullBox<P, F> where
    P: PartialBox<ParentData=FullBoxData<F>> + FullBoxInfo,
    F: FlagTrait {
    type ParentData = ();
    type ThisData = FullBoxData<F>;

    fn byte_size(&self) -> usize {
        let version = 0;
        let flags = F::default();
        FullBoxData { version, flags }.byte_size() + self.inner.byte_size()
    }

    const ID: BoxType = P::ID;
}

#[async_trait]
impl<P, F> PartialBoxRead for FullBox<P, F> where
    P: PartialBox<ParentData=FullBoxData<F>> + PartialBoxRead + FullBoxInfo + Send + Sync,
    F: FlagTrait,{
    async fn read_data<R: ReadMp4>(_: Self::ParentData, reader: &mut R) -> Result<Self, MP4Error> {
        let data = FullBoxData::read(reader).await?;
        let inner = P::read_data(data, reader).await?;
        Ok(Self { inner })
    }

    async fn read_child<R: ReadMp4>(&mut self, header: BoxHeader, reader: &mut R) -> Result<(), MP4Error> {
        self.inner.read_child(header, reader).await
    }
}

#[async_trait]
impl<P, F> PartialBoxWrite for FullBox<P, F> where
    P: PartialBox<ParentData=FullBoxData<F>> + PartialBoxWrite + FullBoxInfo<Flag=F> + Send + Sync,
    F: FlagTrait, {

    async fn write_data<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        let version = self.inner.version();
        let flags = self.inner.flags();
        count += FullBoxData::<F>{ version, flags }.write(writer).await?;
        count += self.inner.write_data(writer).await?;
        Ok(count)
    }

    async fn write_children<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        self.inner.write_children(writer).await
    }
}

impl<P, F> Deref for FullBox<P, F>
    where
        P: PartialBox<ParentData=FullBoxData<F>> + FullBoxInfo,
        F: FlagTrait,
{
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
