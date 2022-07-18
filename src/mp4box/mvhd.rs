
use std::mem;
use std::mem::size_of;
use chrono::{DateTime, Duration, TimeZone, Utc};
use fixed::types::{I16F16, I8F8};
use crate::matrix::MP4Matrix;
use crate::mp4box::box_full::{FullBox, FullBoxData, FullBoxInfo};
use crate::mp4box::box_root::MP4Box;
use async_trait::async_trait;
use fixed_macro::fixed;
use crate::bytes_read::{Mp4VersionedReadable, ReadMp4};
use crate::error::MalformedBoxError::UnknownVersion;
use crate::error::MP4Error;
use crate::id::BoxId;
use crate::r#type::BoxType;
use crate::bytes_write::{Mp4VersionedWritable, Mp4Writable, WriteMp4};
use crate::mp4box::box_trait::{PartialBox, PartialBoxRead, PartialBoxWrite};

pub fn base_date() -> DateTime<Utc> {
    Utc.ymd(1904, 1, 1).and_hms(0, 0, 0)
}

#[derive(Copy, Clone, Debug)]
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
impl<F> Mp4VersionedReadable<F> for Mp4DateTime {
    async fn versioned_read<R: ReadMp4>(version: u8, _: F, reader: &mut R) -> Result<Self, MP4Error> {
        Ok(Self(match version {
            0 => reader.read::<u32>().await? as u64,
            _ => reader.read().await?
        }))
    }
}

#[async_trait]
impl<F> Mp4VersionedWritable<F> for Mp4DateTime {
    fn required_version(&self) -> u8 {
        if self.0 > u32::MAX as u64 { 1 } else { 0 }
    }

    fn versioned_byte_size(&self, version: u8, _: F) -> usize {
        match version {
            0 => u32::byte_size(),
            _ => u64::byte_size()
        }
    }

    async fn versioned_write<W: WriteMp4>(&self, version: u8, _: F, writer: &mut W) -> Result<usize, MP4Error> {
        Ok(match version {
            0 => (self.0 as u32).write(writer).await?,
            _ => self.0.write(writer).await?
        })
    }
}

pub type MvhdBox = MP4Box<FullBox<Mvhd, u32>>;

#[derive(Debug, Clone)]
pub struct Mvhd {
    creation_time: Mp4DateTime,
    modification_time: Mp4DateTime,
    /// number of units in a second
    timescale: u32,
    duration: Option<u64>,
    rate: I16F16,
    volume: I8F8,
    matrix: MP4Matrix,
    next_track_id: u32
}

impl Mvhd {
    fn creation_time(&self) -> u64 {
        self.creation_time.signed_duration_since(base_date()).num_seconds() as u64
    }
    fn modification_time(&self) -> u64 {
        self.modification_time.signed_duration_since(base_date()).num_seconds() as u64
    }
}

impl Default for Mvhd {
    fn default() -> Self {
        Self {
            creation_time: Utc::now(),
            modification_time: Utc::now(),
            timescale: 1000,
            duration: None,
            rate: fixed!(1: I16F16),
            volume: fixed!(1: I8F8),
            matrix: Default::default(),
            next_track_id: 1
        }
    }
}

impl FullBoxInfo for Mvhd {
    fn version(&self) -> u8 {
        let large = self.creation_time() > u32::MAX as u64 ||
            self.modification_time() > u32::MAX as u64 ||
            self.duration.map(|it|it > u32::MAX as u64).unwrap_or(false);
        if large { 1 } else { 0 }
    }
}

impl PartialBox for Mvhd {
    type ParentData = FullBoxData;

    fn byte_size(&self) -> usize {
        let version = self.version();
        let mut base = if version == 1 {
            3 * size_of::<u64>() +
                size_of::<u32>()
        } else {
           4 * size_of::<u32>()
        };
        base += mem::size_of_val(&self.rate);
        base += mem::size_of_val(&self.volume);
        base += size_of::<u16>(); // reserved
        base += 2 * size_of::<u32>(); // reserved
        base += MP4Matrix::byte_size();
        base += 6 * size_of::<u32>(); // reserved
        base += mem::size_of_val(&self.next_track_id);
        base
    }

    const ID: BoxType = BoxType::Id(BoxId(*b"mvhd"));
}

#[async_trait]
impl<R: ReadMp4> PartialBoxRead<R> for Mvhd {
    async fn read_data(data: Self::ParentData, reader: &mut R) -> Result<Self, MP4Error> {
        let base_time = base_date();
        let (creation_time, modification_time, timescale, duration) =
        match data.version {
            0 => {
                let (a, b, c, d): (u32, u32, u32, Option<u64>) = (
                reader.read().await?,
                reader.read().await?,
                reader.read().await?,
                Some(reader.read().await?).and_then(|it: u32| {
                    if it == u32::MAX {
                        None
                    } else {
                        Some(it as u64)
                    }
                }),
                );
                (a as i64, b as i64, c, d)
            }
            1 => {
                let (a, b, c, d): (u64, u64, u32, Option<u64>) = (
                reader.read().await?,
                reader.read().await?,
                reader.read().await?,
                Some(reader.read().await?).and_then(|it| {
                    if it == u64::MAX {
                        None
                    } else {
                        Some(it)
                    }
                }),
                );
                (a as i64, b as i64, c, d)
            }
            _ => return Err(UnknownVersion(Self::ID, data.version).into())
        };
        let rate = I16F16::from_bits(reader.read().await?);
        let volume = I8F8::from_bits(reader.read().await?);
        reader.reserve::<u16>().await?;
        reader.reserve::<[u32;2]>().await?;
        let matrix = reader.read().await?;
        reader.reserve::<[u32;6]>().await?;
        let next_track_id = reader.read().await?;

        let creation_time = base_time.clone() + Duration::seconds(creation_time);
        let modification_time = base_time.clone() + Duration::seconds(modification_time);

        Ok(Self {
            creation_time,
            modification_time,
            timescale,
            duration,
            rate,
            volume,
            matrix,
            next_track_id
        })
    }
}

#[async_trait]
impl<W: WriteMp4> PartialBoxWrite<W> for Mvhd {

    async fn write_data(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let version = self.version();
        let mut count = 0;
        if version == 0 {
            count += (self.creation_time() as u32).write(writer).await?;
            count += (self.modification_time() as u32).write(writer).await?;
            count += self.timescale.write(writer).await?;
            count += if let Some(duration) = self.duration {
                (duration as u32).write(writer).await?
            } else {
                u32::MAX.write(writer).await?
            }
        } else {
            count += self.creation_time().write(writer).await?;
            count += self.modification_time().write(writer).await?;
            count += self.timescale.write(writer).await?;
            count += if let Some(duration) = self.duration {
                duration.write(writer).await?
            } else {
                u64::MAX.write(writer).await?
            }
        }
        count += self.rate.to_bits().write(writer).await?;
        count += self.volume.to_bits().write(writer).await?;
        count += writer.reserve::<u16>().await?;
        count += writer.reserve::<[u32;2]>().await?;
        count += self.matrix.write(writer).await?;
        count += writer.reserve::<[u32; 6]>().await?;
        count += self.next_track_id.write(writer).await?;
        Ok(count)
    }
}
