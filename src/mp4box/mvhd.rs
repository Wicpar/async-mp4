
use chrono::{DateTime, Duration, TimeZone, Utc};
use fixed::types::{I16F16, I8F8};
use crate::matrix::MP4Matrix;
use async_trait::async_trait;
use fixed::{FixedI16, FixedI32};
use fixed_macro::fixed;
use crate::bytes_read::{Mp4Readable, Mp4VersionedReadable, ReadMp4};
use crate::bytes_reserve::Mp4Reservable;
use crate::error::MP4Error;
use crate::bytes_write::{FlagTrait, Mp4VersionedWritable, Mp4Writable, WriteMp4};
use crate::full_box;

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

full_box! {
    box (b"mvhd", Mvhd, MvhdBox, u32)
    data {
        creation_time: Mp4DateTime,
        modification_time: Mp4DateTime,
        timescale: u32,
        duration: Mp4Duration,
        rate: I16F16,
        volume: I8F8,
        _r1: u16,
        _r2: [u32; 2],
        matrix: MP4Matrix,
        _r3: [u32; 6],
        next_track_id: u32
    }
}

impl Default for Mvhd {
    fn default() -> Self {
        Self {
            creation_time: Default::default(),
            modification_time: Default::default(),
            timescale: 1000,
            duration: Default::default(),
            rate: fixed!(1: I16F16),
            volume: fixed!(1: I8F8),
            _r1: Default::default(),
            _r2: Default::default(),
            matrix: Default::default(),
            _r3: Default::default(),
            next_track_id: 1
        }
    }
}
