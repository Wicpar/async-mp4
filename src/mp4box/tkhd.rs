use std::mem;
use std::mem::size_of;
use chrono::{DateTime, Duration, Utc};
use crate::id::BoxId;
use crate::mp4box::full_box::{FullBox, FullBoxData, FullBoxInfo};
use crate::mp4box::{PartialBox, PartialBoxRead, PartialBoxWrite};
use crate::mp4box::rootbox::MP4Box;
use crate::r#type::BoxType;
pub use async_trait::async_trait;
use bitregions::bitregions;
use byteorder_async::{BigEndian, ReaderToByteOrder};
use fixed::types::I8F8;
use fixed_macro::fixed;
use futures::{AsyncRead, AsyncSeek};
use crate::bytes_read::ReadMp4;
use crate::error::MalformedBoxError::UnknownVersion;
use crate::error::MP4Error;
use crate::matrix::MP4Matrix;
use crate::mp4box::mvhd::base_date;
use crate::bytes_write::WriteMp4;

pub type TkhdBox = MP4Box<FullBox<Tkhd>>;

bitregions! {
    pub TrakFlags u32 {
        ENABLED:                0b0001,
        IN_MOVIE:               0b0010,
        IN_PREVIEW:             0b0100,
        SIZE_IS_ASPECT_RATIO:   0b1000,
    }
}


#[derive(Debug, Clone)]
pub struct Tkhd {
    flags: TrakFlags,
    creation_time: DateTime<Utc>,
    modification_time: DateTime<Utc>,
    track_id: u32,
    duration: Option<u64>,
    layer: i16,
    alternate_group: i16,
    volume: I8F8,
    matrix: MP4Matrix,
    width: u32,
    height: u32
}

impl Tkhd {
    fn creation_time(&self) -> u64 {
        self.creation_time.signed_duration_since(base_date()).num_seconds() as u64
    }
    fn modification_time(&self) -> u64 {
        self.modification_time.signed_duration_since(base_date()).num_seconds() as u64
    }
}

impl Default for Tkhd {
    fn default() -> Self {
        Self {
            flags: TrakFlags::default() | TrakFlags::ENABLED,
            creation_time: Utc::now(),
            modification_time: Utc::now(),
            track_id: 1,
            duration: None,
            layer: 0,
            alternate_group: 0,
            volume: fixed!(1: I8F8),
            matrix: Default::default(),
            width: 0,
            height: 0
        }
    }
}

impl FullBoxInfo for Tkhd {
    fn version(&self) -> u8 {
        let large = self.creation_time() > u32::MAX as u64 ||
            self.modification_time() > u32::MAX as u64 ||
            self.duration.map(|it|it > u32::MAX as u64).unwrap_or(false);
        if large { 1 } else { 0 }
    }

    fn flags(&self) -> u32 {
        self.flags.into()
    }
}

impl PartialBox for Tkhd {
    type ParentData = FullBoxData;

    fn byte_size(&self) -> usize {
        let version = self.version();
        let mut base = if version == 1 {
            3 * size_of::<u64>() +
            2 * size_of::<u32>()
        } else {
            5 * size_of::<u32>()
        };
        base += size_of::<[u32;2]>(); // reserved
        base += mem::size_of_val(&self.layer);
        base += mem::size_of_val(&self.alternate_group);
        base += mem::size_of_val(&self.volume);
        base += size_of::<u16>(); // reserved
        base += MP4Matrix::byte_size();
        base += mem::size_of_val(&self.width);
        base += mem::size_of_val(&self.height);
        base
    }

    const ID: BoxType = BoxType::Id(BoxId(*b"tkhd"));
}

#[async_trait]
impl<R: ReadMp4> PartialBoxRead<R> for Tkhd {
    async fn read_data(data: Self::ParentData, reader: &mut R) -> Result<Self, MP4Error> {
        let base_time = base_date();
        let (creation_time, modification_time, track_id, _, duration) =
            match data.version {
                0 => {
                    (
                        reader.read_u32().await? as i64,
                        reader.read_u32().await? as i64,
                        reader.read_u32().await?,
                        reader.reserved(size_of::<u32>()).await?,
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
                        reader.reserved(size_of::<u32>()).await?,
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
        reader.reserved(size_of::<[u32;2]>()).await?;
        let layer = reader.read_i16().await?;
        let alternate_group = reader.read_i16().await?;
        let volume = I8F8::from_bits(reader.read_i16().await?);
        reader.reserved(size_of::<u16>()).await?;
        let matrix = MP4Matrix::read(reader).await?;
        let width = reader.read_u32().await?;
        let height = reader.read_u32().await?;

        let creation_time = base_time.clone() + Duration::seconds(creation_time);
        let modification_time = base_time.clone() + Duration::seconds(modification_time);

        Ok(Self {
            flags: data.flags.into(),
            creation_time,
            modification_time,
            track_id,
            duration,
            layer,
            alternate_group,
            volume,
            matrix,
            width,
            height
        })
    }
}

#[async_trait]
impl<W: WriteMp4> PartialBoxWrite<W> for Tkhd {

    async fn write_data(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let version = self.version();
        let mut count = 0;
        if version == 0 {
            count += writer.write_u32(self.creation_time() as _).await?;
            count += writer.write_u32(self.modification_time() as _).await?;
            count += writer.write_u32(self.track_id).await?;
            count += writer.reserved(size_of::<u32>()).await?;
            count += if let Some(duration) = self.duration {
                writer.write_u32(duration as _).await?
            } else {
                writer.write_u32(u32::MAX).await?
            }
        } else {
            count += writer.write_u64(self.creation_time()).await?;
            count += writer.write_u64(self.modification_time()).await?;
            count += writer.write_u32(self.track_id).await?;
            count += writer.reserved(size_of::<u32>()).await?;
            count += if let Some(duration) = self.duration {
                writer.write_u64(duration).await?
            } else {
                writer.write_u64(u64::MAX).await?
            }
        }
        count += writer.reserved(size_of::<[u32; 2]>()).await?; // reserved
        count += writer.write_i16(self.layer).await?;
        count += writer.write_i16(self.alternate_group).await?;
        count += writer.write_i16(self.volume.to_bits()).await?;
        count += writer.reserved(size_of::<u16>()).await?; // reserved
        count += self.matrix.write(writer).await?;
        count += writer.write_u32(self.width).await?;
        count += writer.write_u32(self.height).await?;
        Ok(count)
    }
}
