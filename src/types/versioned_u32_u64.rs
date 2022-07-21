use std::ops::Deref;
use async_trait::async_trait;
use crate::bytes_read::{Mp4VersionedReadable, ReadMp4};
use crate::bytes_reserve::Mp4Reservable;
use crate::bytes_write::{FlagTrait, Mp4VersionedWritable, Mp4Writable, WriteMp4};
use crate::error::MP4Error;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VersionedU32U64(pub u64);

impl From<u32> for VersionedU32U64 {
    fn from(t: u32) -> Self {
        Self(t as u64)
    }
}
impl From<u64> for VersionedU32U64 {
    fn from(t: u64) -> Self {
        Self(t)
    }
}
impl From<VersionedU32U64> for u64 {
    fn from(t: VersionedU32U64) -> Self {
        t.0
    }
}

impl Deref for VersionedU32U64 {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<F: FlagTrait> Mp4VersionedReadable<F> for VersionedU32U64 {
    async fn versioned_read<R: ReadMp4 + ?Sized>(version: u8, _: F, reader: &mut R) -> Result<Self, MP4Error> {
        Ok(Self(match version {
            0 => reader.read::<u32>().await? as u64,
            _ => reader.read().await?
        }))
    }
}

#[async_trait]
impl<F: FlagTrait> Mp4VersionedWritable<F> for VersionedU32U64 {
    fn required_version(&self) -> u8 {
        if self.0 >= u32::MAX as u64 { 1 } else { 0 }
    }

    fn versioned_byte_size(&self, version: u8, _: F) -> usize {
        match version {
            0 => u32::BYTE_SIZE,
            _ => u64::BYTE_SIZE
        }
    }

    async fn versioned_write<W: WriteMp4>(&self, version: u8, _: F, writer: &mut W) -> Result<usize, MP4Error> {
        Ok(match version {
            0 => (self.0 as u32).write(writer).await?,
            _ => self.0.write(writer).await?
        })
    }
}
