use std::io::SeekFrom;
use std::mem;
use std::mem::size_of;
use std::ops::{Div, Mul};
use chrono::{Date, DateTime, Duration, NaiveDate, NaiveTime, TimeZone, Utc};
use fixed::{FixedI8, FixedU8};
use fixed::types::extra::U8;
use fixed::types::{I16F16, I8F8};
use crate::header::BoxHeader;
use crate::matrix::MP4Matrix;
use crate::mp4box::full_box::{FullBox, FullBoxData, FullBoxInfo};
use crate::mp4box::{PartialBox, PartialBoxRead, PartialBoxWrite};
use crate::mp4box::rootbox::MP4Box;
use async_trait::async_trait;
use byteorder_async::{BigEndian, ReaderToByteOrder, WriteBytesExt};
use fixed_macro::fixed;
use futures::{AsyncRead, AsyncSeek, AsyncSeekExt, AsyncWrite};
use crate::bytes_read::ReadMp4;
use crate::error::MalformedBoxError::UnknownVersion;
use crate::error::MP4Error;
use crate::id::BoxId;
use crate::r#type::BoxType;
use crate::bytes_write::WriteMp4;

pub fn base_date() -> DateTime<Utc> {
    Utc.ymd(1904, 1, 1).and_hms(0, 0, 0)
}

pub type MvhdBox = MP4Box<FullBox<Mvhd>>;

#[derive(Debug, Clone)]
pub struct Mvhd {
    creation_time: DateTime<Utc>,
    modification_time: DateTime<Utc>,
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
impl<R: AsyncRead + AsyncSeek + Unpin + Send + Sync> PartialBoxRead<R> for Mvhd {
    async fn read_data(data: Self::ParentData, reader: &mut R) -> Result<Self, MP4Error> {
        let base_time = base_date();
        let (creation_time, modification_time, timescale, duration) =
        match data.version {
            0 => {
                (
                reader.read_u32().await? as i64,
                reader.read_u32().await? as i64,
                reader.read_u32().await?,
                Some(reader.read_u32().await?).and_then(|it| {
                    if it == u32::MAX {
                        None
                    } else {
                        Some(it as u64)
                    }
                }),
                )
            }
            1 => {
                (
                reader.read_u64().await? as i64,
                reader.read_u64().await? as i64,
                reader.read_u32().await?,
                Some(reader.read_u64().await?).and_then(|it| {
                    if it == u64::MAX {
                        None
                    } else {
                        Some(it as u64)
                    }
                }),
                )
            }
            _ => return Err(UnknownVersion(Self::ID, data.version).into())
        };
        let rate = I16F16::from_bits(reader.byte_order().read_i32::<BigEndian>().await?);
        let volume = I8F8::from_bits(reader.byte_order().read_i16::<BigEndian>().await?);
        reader.reserved(size_of::<u16>()).await?;
        reader.reserved(size_of::<[u32;2]>()).await?;
        let matrix = MP4Matrix::read(reader).await?;
        reader.reserved(size_of::<[u32;6]>()).await?;
        let next_track_id = reader.byte_order().read_u32::<BigEndian>().await?;

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
            count += writer.write_u32(self.creation_time() as _).await?;
            count += writer.write_u32(self.modification_time() as _).await?;
            count += writer.write_u32(self.timescale).await?;
            count += if let Some(duration) = self.duration {
                writer.write_u32(duration as _).await?
            } else {
                writer.write_u32(u32::MAX).await?
            }
        } else {
            count += writer.write_u64(self.creation_time()).await?;
            count += writer.write_u64(self.modification_time()).await?;
            count += writer.write_u32(self.timescale).await?;
            count += if let Some(duration) = self.duration {
                writer.write_u64(duration).await?
            } else {
                writer.write_u64(u64::MAX).await?
            }
        }
        count += writer.write_i32(self.rate.to_bits()).await?;
        count += writer.write_i16(self.volume.to_bits()).await?;
        count += writer.reserved(size_of::<u16>()).await?;
        count += writer.reserved(size_of::<[u32;2]>()).await?;
        count += self.matrix.write(writer).await?;
        count += writer.reserved(size_of::<[u32; 6]>()).await?;
        count += writer.write_u32(self.next_track_id).await?;
        Ok(count)
    }
}
