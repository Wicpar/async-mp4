use async_trait::async_trait;
use chrono::{DateTime, Duration, TimeZone, Utc};
use crate::bytes_read::{Mp4VersionedReadable, ReadMp4};
use crate::bytes_reserve::Mp4Reservable;
use crate::bytes_write::{FlagTrait, Mp4VersionedWritable, Mp4Writable, WriteMp4};
use crate::error::MP4Error;

pub fn base_date() -> DateTime<Utc> {
    Utc.ymd(1904, 1, 1).and_hms(0, 0, 0)
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Mp4DateTime(u64);

impl Default for Mp4DateTime {
    fn default() -> Self {
        Utc::now().into()
    }
}


impl From<DateTime<Utc>> for Mp4DateTime {
    fn from(date: DateTime<Utc>) -> Self {
        Self(date.signed_duration_since(base_date()).num_seconds() as u64)
    }
}

impl From<Mp4DateTime> for DateTime<Utc> {
    fn from(date: Mp4DateTime) -> Self {
        base_date() + Duration::seconds(date.0 as i64)
    }
}

#[async_trait]
impl<F: FlagTrait> Mp4VersionedReadable<F> for Mp4DateTime {
    async fn versioned_read<R: ReadMp4 + ?Sized>(version: u8, _: F, reader: &mut R) -> Result<Self, MP4Error> {
        Ok(Self(match version {
            0 => reader.read::<u32>().await? as u64,
            _ => reader.read().await?
        }))
    }
}

#[async_trait]
impl<F: FlagTrait> Mp4VersionedWritable<F> for Mp4DateTime {
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
