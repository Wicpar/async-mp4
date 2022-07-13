use crate::r#box::full_box::{FullBox, FullBoxData, FullBoxInfo};
use crate::r#box::r#box::MP4Box;
use bitregions::bitregions;
use crate::r#box::PartialBox;
use crate::r#type::BoxType;
use crate::write_bytes::WriteMp4Ext;

pub type TrexBox = MP4Box<FullBox<Trex>>;

pub const TREX: [u8;4] = *b"trex";

bitregions! {
    pub SampleFlags u32 {
        IS_LEADING:                 0b0000000000000000__0_000_00_00_00_11_0000,
        SAMPLE_DEPENDS_ON:          0b0000000000000000__0_000_00_00_11_00_0000,
        SAMPLE_IS_DEPENDED_ON:      0b0000000000000000__0_000_00_11_00_00_0000,
        SAMPLE_HAS_REDUNDANCY:      0b0000000000000000__0_000_11_00_00_00_0000,
        SAMPLE_PADDING_VALUE:       0b0000000000000000__0_111_00_00_00_00_0000,
        SAMPLE_IS_NON_SYNC_SAMPLE:  0b0000000000000000__1_000_00_00_00_00_0000,
        SAMPLE_IS_NON_SYNC_SAMPLE:  0b1111111111111111__0_000_00_00_00_00_0000,
    }
}

#[derive(Debug, Clone, Default)]
pub struct Trex {
    pub track_id: u32,
    pub default_sample_description_index: u32
}

impl FullBoxInfo for Trex {}

impl PartialBox for Trex {
    type ParentData = FullBoxData;

    fn byte_size(&self) -> usize {
        0
    }

    fn id() -> BoxType {
        TREX.into()
    }
}

#[async_trait]
impl<R> PartialBoxRead<R> for Trex
    where
        R: AsyncRead + AsyncSeek + Unpin + Send + Sync {

    async fn read_data(_: Self::ParentData, _: &mut R) -> Result<Self, MP4Error> {
        Ok(Default::default())
    }

    async fn read_child(&mut self, header: BoxHeader, reader: &mut R) -> Result<(), MP4Error> {
        Ok(())
    }
}

#[async_trait]
impl<W: WriteMp4Ext> PartialBoxWrite<W> for Trex
    where
        W: AsyncWrite + Unpin + Send + Sync {
    async fn write_children(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        Ok(count)
    }
}
