use std::io::SeekFrom;
use std::mem;
use std::ops::{Div, Mul};
use chrono::{Date, DateTime, Duration, NaiveDate, NaiveTime, TimeZone, Utc};
use fixed::{FixedI8, FixedU8};
use fixed::types::extra::U8;
use fixed::types::{I16F16, I8F8};
use crate::header::BoxHeader;
use crate::matrix::MP4Matrix;
use crate::r#box::full_box::{FullBox, FullBoxData, FullBoxInfo};
use crate::r#box::{PartialBox, PartialBoxRead, PartialBoxWrite};
use crate::r#box::r#box::MP4Box;
use async_trait::async_trait;
use byteorder_async::{BigEndian, ReaderToByteOrder, WriteBytesExt};
use fixed_macro::fixed;
use futures::{AsyncRead, AsyncSeek, AsyncSeekExt, AsyncWrite};
use crate::error::MalformedBoxError::UnknownVersion;
use crate::error::MP4Error;
use crate::r#type::BoxType;
use crate::write_bytes::WriteMp4Ext;

fn base_date() -> DateTime<Utc> {
    Utc.ymd(1904, 1, 1).and_hms(0, 0, 0)
}

pub type MvhdBox = MP4Box<FullBox<Mvhd>>;
pub const MVHD: [u8;4] = *b"mvhd";

#[derive(Debug, Clone)]
pub struct Mvhd {
    creation_time: DateTime<Utc>,
    modification_time: DateTime<Utc>,
    /// number of units in a second
    timescale: u32,
    duration: Option<Duration>,
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
    fn duration(&self) -> Option<u64> {
        Some(self.duration?.mul(self.timescale as _).num_seconds() as _)
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
            self.duration().map(|it|it > u32::MAX as u64).unwrap_or(false);
        if large { 1 } else { 0 }
    }
}

impl PartialBox for Mvhd {
    type ParentData = FullBoxData;

    fn byte_size(&self) -> usize {
        let version = self.version();
        let mut base = if version == 1 {
            3 * mem::size_of::<u64>() +
                mem::size_of::<u32>()
        } else {
           4 * mem::size_of::<u32>()
        };
        base += mem::size_of_val(&self.rate);
        base += mem::size_of_val(&self.volume);
        base += 10; // reserved
        base += MP4Matrix::byte_size();
        base += 6 * 4; // reserved
        base += mem::size_of_val(&self.next_track_id);
        base
    }

    fn id() -> BoxType {
        MVHD.into()
    }
}

#[async_trait]
impl<R: AsyncRead + AsyncSeek + Unpin + Send + Sync> PartialBoxRead<R> for Mvhd {
    async fn read_data(data: Self::ParentData, reader: &mut R) -> Result<Self, MP4Error> {
        let base_time = base_date();
        let (creation_time, modification_time, timescale, duration) =
        match data.version {
            0 => {
                (
                reader.byte_order().read_u32::<BigEndian>().await? as i64,
                reader.byte_order().read_u32::<BigEndian>().await? as i64,
                reader.byte_order().read_u32::<BigEndian>().await?,
                Some(reader.byte_order().read_u32::<BigEndian>().await?).and_then(|it| {
                    if it == u32::MAX {
                        None
                    } else {
                        Some(it as i64)
                    }
                }),
                )
            }
            1 => {
                (
                reader.byte_order().read_u64::<BigEndian>().await? as i64,
                reader.byte_order().read_u64::<BigEndian>().await? as i64,
                reader.byte_order().read_u32::<BigEndian>().await?,
                Some(reader.byte_order().read_u64::<BigEndian>().await?).and_then(|it| {
                    if it == u64::MAX {
                        None
                    } else {
                        Some(it as i64)
                    }
                }),
                )
            }
            _ => return Err(UnknownVersion(Self::id(), data.version).into())
        };
        let rate = I16F16::from_bits(reader.byte_order().read_i32::<BigEndian>().await?);
        let volume = I8F8::from_bits(reader.byte_order().read_i16::<BigEndian>().await?);
        let duration = duration.map(|it| Duration::seconds(it).div(timescale as _));
        reader.seek(SeekFrom::Current(2 + 2 * 4)).await?; // reserved
        let matrix = MP4Matrix::read(reader).await?;
        reader.seek(SeekFrom::Current(6 * 4)).await?; // reserved
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
impl<W: WriteMp4Ext> PartialBoxWrite<W> for Mvhd {

    async fn write_data(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let version = self.version();
        let mut count = 0;
        if version == 0 {
            count += writer.write_u32::<BigEndian>(self.creation_time() as _).await?;
            count += writer.write_u32::<BigEndian>(self.modification_time() as _).await?;
            count += writer.write_u32::<BigEndian>(self.timescale).await?;
            count += if let Some(duration) = self.duration() {
                writer.write_u32::<BigEndian>(duration as _).await?
            } else {
                writer.write_u32::<BigEndian>(u32::MAX).await?
            }
        } else {
            count += writer.write_u64::<BigEndian>(self.creation_time()).await?;
            count += writer.write_u64::<BigEndian>(self.modification_time()).await?;
            count += writer.write_u32::<BigEndian>(self.timescale).await?;
            count += if let Some(duration) = self.duration() {
                writer.write_u64::<BigEndian>(duration).await?
            } else {
                writer.write_u64::<BigEndian>(u64::MAX).await?
            }
        }
        count += writer.write_i32::<BigEndian>(self.rate.to_bits()).await?;
        count += writer.write_i16::<BigEndian>(self.volume.to_bits()).await?;
        count += writer.write_all(&[0;2 + 2 * 4]).await?; // reserved
        count += self.matrix.write(writer).await?;
        count += writer.write_all(&[0;6 * 4]).await?;
        count += writer.write_u32::<BigEndian>(self.next_track_id).await?;
        Ok(count)
    }
}
