use std::fs::read;
use std::mem::size_of_val;
use std::mem::size_of;
use chrono::{DateTime, Duration, TimeZone, Utc};
use fixed::types::{I16F16, I8F8};
use crate::matrix::MP4Matrix;
use crate::mp4box::box_full::{FullBox, FullBoxData, FullBoxInfo};
use crate::mp4box::box_root::MP4Box;
use async_trait::async_trait;
use fixed_macro::fixed;
use crate::bytes_read::{Mp4Readable, ReadMp4};
use crate::error::MalformedBoxError::UnknownVersion;
use crate::error::MP4Error;
use crate::id::BoxId;
use crate::r#type::BoxType;
use crate::bytes_write::{Mp4Writable, WriteMp4};
use crate::mp4box::box_trait::{PartialBox, PartialBoxRead, PartialBoxWrite};

#[repr(u16)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum VideoGraphicsMode {
    Copy = 0
}
#[async_trait]
impl Mp4Writable for VideoGraphicsMode {
    fn byte_size(&self) -> usize {
        (*self as u16).byte_size()
    }

    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        (*self as u16).write(writer).await
    }
}
#[async_trait]
impl Mp4Readable for VideoGraphicsMode {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        Ok(u16::read(reader).await? as VideoGraphicsMode)
    }
}


pub type VmhdBox = MP4Box<FullBox<Vmhd>>;

#[derive(Debug, Clone)]
pub struct Vmhd {
    pub mode: VideoGraphicsMode,
    pub color: [u16; 3]
}


impl Default for Vmhd {
    fn default() -> Self {
        Self {
            mode: VideoGraphicsMode::Copy,
            color: [0, 0, 0]
        }
    }
}

impl FullBoxInfo for Vmhd {
    fn version(&self) -> u8 {
        0
    }

    fn flags(&self) -> u32 {
        1
    }
}

impl PartialBox for Vmhd {
    type ParentData = FullBoxData;

    fn byte_size(&self) -> usize {
        let mut base = 0;
        base += size_of_val(&self.mode);
        base += size_of_val(&self.color);
        base
    }

    const ID: BoxType = BoxType::Id(BoxId(*b"vmhd"));
}

#[async_trait]
impl<R: ReadMp4> PartialBoxRead<R> for Vmhd {
    async fn read_data(_: Self::ParentData, reader: &mut R) -> Result<Self, MP4Error> {
        let mode = reader.read().await?;
        let color= reader.read().await?;
        Ok(Self {
            mode, color
        })
    }
}

#[async_trait]
impl<W: WriteMp4> PartialBoxWrite<W> for Vmhd {

    async fn write_data(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        count += self.mode.write(writer).await?;
        count += self.color.write(writer).await?;
        Ok(count)
    }
}
