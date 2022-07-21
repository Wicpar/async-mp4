use async_trait::async_trait;
use crate::bytes_read::{Mp4VersionedReadable, ReadMp4};
use crate::bytes_reserve::Mp4Reservable;
use crate::bytes_write::{FlagTrait, Mp4VersionedWritable, Mp4Writable, WriteMp4};
use crate::error::{MP4Error};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum VersionedSignedU32 {
    Unsigned(u32),
    Signed(i32)
}

impl Default for VersionedSignedU32 {
    fn default() -> Self {
        Self::Unsigned(0)
    }
}

impl From<i32> for VersionedSignedU32 {
    fn from(t: i32) -> Self {
        Self::Signed(t)
    }
}
impl From<u32> for VersionedSignedU32 {
    fn from(t: u32) -> Self {
        Self::Unsigned(t)
    }
}

#[async_trait]
impl<F: FlagTrait> Mp4VersionedReadable<F> for VersionedSignedU32 {
    async fn versioned_read<R: ReadMp4 + ?Sized>(version: u8, _: F, reader: &mut R) -> Result<Self, MP4Error> {
        Ok(match version {
            0 => Self::Unsigned(reader.read().await?),
            _ => Self::Signed(reader.read().await?)
        })
    }
}

#[async_trait]
impl<F: FlagTrait> Mp4VersionedWritable<F> for VersionedSignedU32 {
    fn required_version(&self) -> u8 {
        match self {
            VersionedSignedU32::Unsigned(_) => 0,
            VersionedSignedU32::Signed(_) => 1
        }
    }

    fn versioned_byte_size(&self, _: u8, _: F) -> usize {
        u32::BYTE_SIZE
    }

    async fn versioned_write<W: WriteMp4>(&self, version: u8, _: F, writer: &mut W) -> Result<usize, MP4Error> {
        Ok(match version {
            0 => match self {
                VersionedSignedU32::Unsigned(it) => it.write(writer).await?,
                VersionedSignedU32::Signed(it) =>  {
                    if *it < 0 {
                        return Err(MP4Error::Custom("VersionedSignedU32: Trying to insert negative int into unsigned int".into()))
                    } else {
                        (*it as u32).write(writer).await?
                    }
                }
            },
            _ => match self {
                VersionedSignedU32::Unsigned(it) => (*it as i32).write(writer).await?,
                VersionedSignedU32::Signed(it) => it.write(writer).await?
            }
        })
    }
}
