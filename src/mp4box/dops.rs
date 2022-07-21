use futures::AsyncReadExt;
use crate::{base_box};
use crate::bytes_read::ReadMp4;
use crate::bytes_write::{Mp4Writable, WriteMp4};
use crate::error::MP4Error;
use crate::id::BoxId;
use crate::mp4box::box_root::MP4Box;
use crate::mp4box::box_trait::{PartialBox, PartialBoxRead, PartialBoxWrite};
use crate::r#type::BoxType;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ChannelMapping {
    pub stream_count: u8,
    pub coupled_count: u8,
    pub channel_mapping: Vec<u8>
}

impl ChannelMapping {
    async fn read<R: ReadMp4>(reader: &mut R, channel_count: u8) -> Result<Self, MP4Error> {
        let stream_count = reader.read().await?;
        let coupled_count = reader.read().await?;
        let mut channel_mapping = vec![0u8; channel_count as usize];
        reader.read_exact(&mut channel_mapping).await?;
        Ok(Self {
            stream_count,
            coupled_count,
            channel_mapping
        })
    }

    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        count += self.stream_count.write(writer).await?;
        count += self.coupled_count.write(writer).await?;
        count += self.channel_mapping.write(writer).await?;
        Ok(count)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ChannelMappingFamily {
    Family0 {
        stereo: bool
    },
    Family1(ChannelMapping),
    Unknown(ChannelMapping)
}

impl ChannelMappingFamily {
    fn byte_size(&self) -> usize {
        match self {
            ChannelMappingFamily::Family0 { .. } => 2,
            ChannelMappingFamily::Family1(mapping) => 4 + mapping.channel_mapping.len(),
            ChannelMappingFamily::Unknown(mapping) => 4 + mapping.channel_mapping.len(),
        }
    }

    fn get_channel_count(&self) -> u8 {
        match self {
            ChannelMappingFamily::Family0 { stereo } => if *stereo { 2 } else { 1 },
            ChannelMappingFamily::Family1(mapping) => mapping.channel_mapping.len() as u8,
            ChannelMappingFamily::Unknown(mapping) => mapping.channel_mapping.len() as u8
        }
    }

    fn get_channel_family(&self) -> u8 {
        match self {
            ChannelMappingFamily::Family0 { .. } => 0,
            ChannelMappingFamily::Family1(_) => 1,
            ChannelMappingFamily::Unknown(_) => 255
        }
    }

    async fn read<R: ReadMp4>(reader: &mut R, channel_count: u8) -> Result<Self, MP4Error> {
        let family: u8 = reader.read().await?;
        Ok(match family {
            0 => Self::Family0 { stereo: channel_count == 2 },
            1 => Self::Family1(ChannelMapping::read(reader, channel_count).await?),
            _ => Self::Unknown(ChannelMapping::read(reader, channel_count).await?)
        })
    }

    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        count += self.get_channel_family().write(writer).await?;
        count += match self {
            ChannelMappingFamily::Family0 { .. } => 0,
            ChannelMappingFamily::Family1(mapping) => {
                debug_assert!(mapping.channel_mapping.len() <= 8, "Opus Family1 cannot have more than 8 output channels");
                mapping.write(writer).await?
            },
            ChannelMappingFamily::Unknown(mapping) => {
                debug_assert!(mapping.channel_mapping.len() <= 255, "Opus Unknown Family cannot have more than 255 output channels");
                mapping.write(writer).await?
            },
        };
        Ok(count)
    }
}


pub type DOpsBox = MP4Box<DOps>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct DOps {
    pub version: u8,
    pub pre_skip: u16,
    pub input_sample_rate: u32,
    pub output_gain: i16,
    pub channel_mapping_family: ChannelMappingFamily
}

impl PartialBox for DOps {
    type ParentData = ();
    type ThisData = ();

    fn byte_size(&self) -> usize {
        self.version.byte_size() +
            self.pre_skip.byte_size() +
            self.input_sample_rate.byte_size() +
            self.output_gain.byte_size()+
            self.channel_mapping_family.byte_size()
    }

    const ID: BoxType = BoxType::Id(BoxId(*b"dOps"));
}

#[async_trait::async_trait]
impl PartialBoxRead for DOps {
    async fn read_data<R: ReadMp4>(_: Self::ParentData, reader: &mut R) -> Result<Self, MP4Error> {
        let version = reader.read().await?;
        let output_channel_count = reader.read().await?;
        let pre_skip = reader.read().await?;
        let input_sample_rate = reader.read().await?;
        let output_gain = reader.read().await?;
        let channel_mapping_family = ChannelMappingFamily::read(reader, output_channel_count).await?;
        Ok(Self {
            version,
            pre_skip,
            input_sample_rate,
            output_gain,
            channel_mapping_family
        })
    }
}

#[async_trait::async_trait]
impl PartialBoxWrite for DOps {

    async fn write_data<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        count += self.version.write(writer).await?;
        count += self.channel_mapping_family.get_channel_count().write(writer).await?;
        count += self.pre_skip.write(writer).await?;
        count += self.input_sample_rate.write(writer).await?;
        count += self.output_gain.write(writer).await?;
        count += self.channel_mapping_family.write(writer).await?;
        Ok(count)
    }
}
