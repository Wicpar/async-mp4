use crate::bytes_read::ReadMp4;
use crate::bytes_reserve::Mp4Reservable;
use crate::bytes_write::{Mp4VersionedWritable, Mp4Writable, WriteMp4};
use crate::error::MP4Error;
use crate::id::BoxId;
use crate::mp4box::box_full::{FullBox, FullBoxData, FullBoxInfo};
use crate::mp4box::box_root::MP4Box;
use crate::mp4box::box_trait::{PartialBox, PartialBoxRead, PartialBoxWrite};
use crate::r#type::BoxType;
use crate::types::array::Mp4Array;


pub type StszBox = MP4Box<FullBox<Stsz, u32>>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Stsz {
    Simple {
        sample_size: u32,
        sample_count: u32,
    },
    Advanced {
        sample_sizes: Mp4Array<u32, u32>
    }
}

impl FullBoxInfo for Stsz {
    type Flag = u32;
}

impl PartialBox for Stsz {
    type ParentData = FullBoxData<u32>;
    type ThisData = ();

    fn byte_size(&self) -> usize {
        match self {
            Stsz::Simple { sample_size, sample_count } => {
                sample_size.byte_size() + sample_count.byte_size()
            }
            Stsz::Advanced { sample_sizes } => {
                u32::BYTE_SIZE + sample_sizes.byte_size()
            }
        }
    }

    const ID: BoxType = BoxType::Id(BoxId(*b"stsz"));
}

#[async_trait::async_trait]
impl PartialBoxRead for Stsz {
    async fn read_data<R: ReadMp4>(parent: Self::ParentData, reader: &mut R) -> Result<Self, MP4Error> {
        let version = parent.version;
        let flags = parent.flags;
        let sample_size = reader.versioned_read(version, flags).await?;
        Ok(if sample_size == 0u32 {
            Self::Advanced {
                sample_sizes: reader.versioned_read(version, flags).await?
            }
        } else {
            Self::Simple {
                sample_size,
                sample_count: reader.versioned_read(version, flags).await?,
            }
        })
    }
}

impl PartialBoxWrite for Stsz {
    fn write_data<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let version = self.version();
        let flags = self.flags();
        let mut count = 0;
        match self {
            Stsz::Simple { sample_size, sample_count } => {
                count += sample_size.versioned_write(version, flags, writer)?;
                count += sample_count.versioned_write(version, flags, writer)?;
            }
            Stsz::Advanced { sample_sizes } => {
                count += 0u32.versioned_write(version, flags, writer)?;
                count += sample_sizes.versioned_write(version, flags, writer)?;
            }
        }
        Ok(count)
    }
}

impl Default for Stsz {
    fn default() -> Self {
        Self::Simple {
            sample_size: 0,
            sample_count: 0
        }
    }
}
