use std::mem;
use std::mem::size_of;
use chrono::{DateTime, Duration, Utc};
use crate::id::BoxId;
use crate::mp4box::box_full::{FullBox, FullBoxData, FullBoxInfo};
use crate::mp4box::box_root::MP4Box;
use crate::r#type::BoxType;
pub use async_trait::async_trait;
use bitregions::bitregions;
use fixed::types::I8F8;
use fixed_macro::fixed;
use crate::bytes_read::ReadMp4;
use crate::bytes_reserve::Mp4Reservable;
use crate::error::MalformedBoxError::UnknownVersion;
use crate::error::MP4Error;
use crate::matrix::MP4Matrix;
use crate::mp4box::mvhd::base_date;
use crate::bytes_write::{Mp4Writable, WriteMp4};
use crate::mp4box::box_trait::{PartialBox, PartialBoxRead, PartialBoxWrite};

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
            <[u64;3]>::BYTE_SIZE + <[u32;2]>::BYTE_SIZE
        } else {
            <[u32;5]>::BYTE_SIZE
        };
        base += <[u32;2]>::BYTE_SIZE; // reserved
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
                        reader.read::<u32>().await? as i64,
                        reader.read::<u32>().await? as i64,
                        reader.read::<u32>().await?,
                        reader.reserve::<u32>().await?,
                        Some(reader.read::<u32>().await?).and_then(|it| {
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
                        reader.read::<u64>().await? as i64,
                        reader.read::<u64>().await? as i64,
                        reader.read::<u32>().await?,
                        reader.reserve::<u32>().await?,
                        Some(reader.read::<u64>().await?).and_then(|it| {
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
        reader.reserve::<[u32;2]>().await?;
        let layer = reader.read().await?;
        let alternate_group = reader.read().await?;
        let volume = I8F8::from_bits(reader.read().await?);
        reader.reserve::<u16>().await?;
        let matrix = reader.read().await?;
        let width = reader.read().await?;
        let height = reader.read().await?;

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
            count += (self.creation_time() as u32).write(writer).await?;
            count += (self.modification_time() as u32).write(writer).await?;
            count += self.track_id.write(writer).await?;
            count += writer.reserve::<u32>().await?;
            count += if let Some(duration) = self.duration {
                (duration as u32).write(writer).await?
            } else {
                u32::MAX.write(writer).await?
            }
        } else {
            count += self.creation_time().write(writer).await?;
            count += self.modification_time().write(writer).await?;
            count += self.track_id.write(writer).await?;
            count += writer.reserve::<u32>().await?;
            count += if let Some(duration) = self.duration {
                duration.write(writer).await?
            } else {
                u64::MAX.write(writer).await?
            }
        }
        count += writer.reserve::<[u32; 2]>().await?;
        count += self.layer.write(writer).await?;
        count += self.alternate_group.write(writer).await?;
        count += self.volume.to_bits().write(writer).await?;
        count += writer.reserve::<u16>().await?;
        count += self.matrix.write(writer).await?;
        count += self.width.write(writer).await?;
        count += self.height.write(writer).await?;
        Ok(count)
    }
}
