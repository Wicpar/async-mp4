use async_trait::async_trait;
use crate::bytes_read::{Mp4VersionedReadable, ReadMp4};
use crate::bytes_reserve::Mp4Reservable;
use crate::bytes_write::{FlagTrait, Mp4VersionedWritable, Mp4Writable, WriteMp4};
use crate::error::MP4Error;

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Mp4Duration(pub Option<u64>);

#[async_trait]
impl<F: FlagTrait> Mp4VersionedReadable<F> for Mp4Duration {
    async fn versioned_read<R: ReadMp4 + ?Sized>(version: u8, _: F, reader: &mut R) -> Result<Self, MP4Error> {
        Ok(Self(match version {
            0 => match reader.read::<u32>().await? {
                u32::MAX => None,
                value=> Some(value as u64)
            },
            _ => match reader.read().await? {
                u64::MAX => None,
                value=> Some(value)
            }
        }))
    }
}

#[async_trait]
impl<F: FlagTrait> Mp4VersionedWritable<F> for Mp4Duration {
    fn required_version(&self) -> u8 {
        if let Some(duration) = self.0 {
           if duration >= u32::MAX as u64 { 1 } else { 0 }
        } else {
            0
        }
    }

    fn versioned_byte_size(&self, version: u8, _: F) -> usize {
        match version {
            0 => u32::BYTE_SIZE,
            _ => u64::BYTE_SIZE
        }
    }

    async fn versioned_write<W: WriteMp4>(&self, version: u8, _: F, writer: &mut W) -> Result<usize, MP4Error> {
        Ok(match version {
            0 => if let Some(value) = self.0 { value as u32} else { u32::MAX }.write(writer).await?,
            _ => if let Some(value) = self.0 { value } else { u64::MAX }.write(writer).await?
        })
    }
}
