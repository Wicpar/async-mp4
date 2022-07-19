use async_trait::async_trait;
use fixed::{FixedI16, FixedI32};
use crate::bytes_read::{Mp4Readable, ReadMp4};
use crate::bytes_write::{Mp4Writable, WriteMp4};
use crate::error::MP4Error;

#[async_trait]
impl<Frac> Mp4Readable for FixedI32<Frac> {
    async fn read<R: ReadMp4 + ?Sized>(reader: &mut R) -> Result<Self, MP4Error> {
        Ok(Self::from_bits(reader.read().await?))
    }
}

#[async_trait]
impl<Frac: Send + Sync> Mp4Writable for FixedI32<Frac> {
    fn byte_size(&self) -> usize {
        self.to_bits().byte_size()
    }

    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        self.to_bits().write(writer).await
    }
}

#[async_trait]
impl<Frac> Mp4Readable for FixedI16<Frac> {
    async fn read<R: ReadMp4 + ?Sized>(reader: &mut R) -> Result<Self, MP4Error> {
        Ok(Self::from_bits(reader.read().await?))
    }
}

#[async_trait]
impl<Frac: Send + Sync> Mp4Writable for FixedI16<Frac> {
    fn byte_size(&self) -> usize {
        self.to_bits().byte_size()
    }

    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        self.to_bits().write(writer).await
    }
}
