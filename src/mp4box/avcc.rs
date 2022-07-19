use crate::{base_box, mp4_data};
use crate::types::array::Mp4Array;
use crate::types::padded_byte::PaddedByte;

mp4_data! {
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub struct AVCDecoderConfigurationRecord {
        pub configuartion_version: u8,
        pub profile_indication: u8,
        pub profile_compatibility: u8,
        pub level_indication: u8,
        pub length_size_minus_one: PaddedByte<6>,
        pub sps: Mp4Array<PaddedByte<3>, Mp4Array<u16, u8>>,
        pub pps: Mp4Array<u8, Mp4Array<u16, u8>>,
        // Todo: profile idc ext (14496-15 § 5.2.4.1.1)
    }
}

base_box! {
    box (b"avcC", AvcC, AvcCBox) data {
        avc_config: AVCDecoderConfigurationRecord
    } children {

    }
}
