use async_trait::async_trait;
use chrono::{DateTime, Duration, TimeZone, Utc};
use crate::bytes_read::{Mp4VersionedReadable, ReadMp4};
use crate::bytes_write::{FlagTrait, Mp4VersionedWritable, WriteMp4};
use crate::error::MP4Error;
use crate::types::versioned_u32_u64::VersionedU32U64;

pub fn base_date() -> DateTime<Utc> {
    Utc.ymd(1904, 1, 1).and_hms(0, 0, 0)
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Mp4DateTime(VersionedU32U64);

impl Default for Mp4DateTime {
    fn default() -> Self {
        Utc::now().into()
    }
}


impl From<DateTime<Utc>> for Mp4DateTime {
    fn from(date: DateTime<Utc>) -> Self {
        Self(VersionedU32U64(date.signed_duration_since(base_date()).num_seconds() as u64))
    }
}

impl From<Mp4DateTime> for DateTime<Utc> {
    fn from(date: Mp4DateTime) -> Self {
        base_date() + Duration::seconds(date.0.0 as i64)
    }
}

#[async_trait]
impl<F: FlagTrait> Mp4VersionedReadable<F> for Mp4DateTime {
    async fn versioned_read<R: ReadMp4 + ?Sized>(version: u8, flags: F, reader: &mut R) -> Result<Self, MP4Error> {
        Ok(Self(reader.versioned_read(version, flags).await?))
    }
}

impl<F: FlagTrait> Mp4VersionedWritable<F> for Mp4DateTime {
    fn required_version(&self) -> u8 {
        <VersionedU32U64 as Mp4VersionedWritable<F>>::required_version(&self.0)
    }

    fn versioned_byte_size(&self, version: u8, flags: F) -> usize {
        self.0.versioned_byte_size(version, flags)
    }

    fn versioned_write<W: WriteMp4>(&self, version: u8, flags: F, writer: &mut W) -> Result<usize, MP4Error> {
        self.0.versioned_write(version, flags, writer)
    }
}
