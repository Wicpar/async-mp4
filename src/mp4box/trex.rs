use crate::mp4box::full_box::{FullBox, FullBoxData, FullBoxInfo};
use crate::mp4box::rootbox::MP4Box;
use bitregions::bitregions;
use crate::mp4box::{PartialBox, PartialBoxRead, PartialBoxWrite};
use crate::r#type::BoxType;
use crate::bytes_write::WriteMp4;
use async_trait::async_trait;
use byteorder_async::{BigEndian, ReaderToByteOrder};
use futures::{AsyncRead, AsyncSeek, AsyncWrite};
use crate::bytes_read::ReadMp4;
use crate::error::MP4Error;
use crate::header::BoxHeader;
use crate::id::BoxId;

pub type TrexBox = MP4Box<FullBox<Trex>>;

#[repr(u8)]
pub enum IsLeading {
    /// the	leading	nature of this sample is unknown
    Unknown = 0,
    /// this sample is a leading sample that has a dependency before the referenced I‐picture (and is therefore	not	decodable)
    LeadingWithDependency = 1,
    /// this sample is not a leading sample
    NotLeading = 2,
    /// this sample is a leading sample that has no dependency before the referenced I‐picture (and is therefore decodable);
    LeadingWithoutDependency = 3,
}

#[repr(u8)]
pub enum SampleDependsOn {
    /// the	dependency	of	this	sample	is	unknown
    Unknown = 0,
    /// 	 this	sample	does	depend	on	others	(not	an	I	picture)
    DependsOn = 1,
    ///	 this	sample	does	not	depend	on	others	(I	picture)
    DoesntDependOn = 2,
    Reserved = 3,
}

#[repr(u8)]
pub enum SampleIsDependedOn {
    /// the	dependency	of	other	samples	on	this	sample	is	unknown
    Unknown = 0,
    ///other	samples	may	depend	on	this	one	(not	disposable)
    DependedOn = 1,
    ///no	other	sample	depends	on	this	one	(disposable)
    NotDependedOn = 2,
    Reserved = 3,
}

#[repr(u8)]
pub enum SampleHasRedundancy {
    ///	 it	is	unknown	whether	there	is	redundant	coding	in	this	sample
    Unknown = 0,
    ///	 there	is	redundant	coding	in	this	sample
    Redundant = 1,
    ///	 there	is	no	redundant	coding	in	this	sample
    NotRedundant = 2,
    Reserved = 3,
}

bitregions! {
    pub SampleFlags u32 {
        IS_LEADING:                     0b0000000000000000__0_000_00_00_00_11_0000,
        SAMPLE_DEPENDS_ON:              0b0000000000000000__0_000_00_00_11_00_0000,
        SAMPLE_IS_DEPENDED_ON:          0b0000000000000000__0_000_00_11_00_00_0000,
        SAMPLE_HAS_REDUNDANCY:          0b0000000000000000__0_000_11_00_00_00_0000,
        SAMPLE_PADDING_VALUE:           0b0000000000000000__0_111_00_00_00_00_0000,
        SAMPLE_IS_NON_SYNC_SAMPLE:      0b0000000000000000__1_000_00_00_00_00_0000,
        SAMPLE_DEGRADATION_PRIORITY:    0b1111111111111111__0_000_00_00_00_00_0000,
    }
}

#[derive(Debug, Clone)]
pub struct Trex {
    pub track_id: u32,
    pub default_sample_description_index: u32,
    pub default_sample_duration: u32,
    pub default_sample_size: u32,
    pub default_sample_flags: SampleFlags,
}

impl FullBoxInfo for Trex {}

impl PartialBox for Trex {
    type ParentData = FullBoxData;

    fn byte_size(&self) -> usize {
        5 * 4
    }

    const ID: BoxType = BoxType::Id(BoxId(*b"trex"));
}

#[async_trait]
impl<R> PartialBoxRead<R> for Trex
    where
        R: AsyncRead + AsyncSeek + Unpin + Send + Sync {

    async fn read_data(_: Self::ParentData, reader: &mut R) -> Result<Self, MP4Error> {
        Ok(Self {
            track_id: reader.read_u32().await?,
            default_sample_description_index: reader.read_u32().await?,
            default_sample_duration: reader.read_u32().await?,
            default_sample_size: reader.read_u32().await?,
            default_sample_flags: reader.read_u32().await?.into()
        })
    }
}

#[async_trait]
impl<W> PartialBoxWrite<W> for Trex
    where
        W: AsyncWrite + Unpin + Send + Sync {
    async fn write_data(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        count += writer.write_u32(self.track_id).await?;
        count += writer.write_u32(self.default_sample_description_index).await?;
        count += writer.write_u32(self.default_sample_duration).await?;
        count += writer.write_u32(self.default_sample_size).await?;
        count += writer.write_u32(self.default_sample_flags.into()).await?;
        Ok(count)
    }
}
