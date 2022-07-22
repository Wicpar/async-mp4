use crate::{base_box, mp4_data};
use crate::types::array::Mp4Array;
use crate::types::padded_byte::PaddedByte;

mp4_data! {
    #[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
    pub struct AVCDecoderConfigurationRecord {
        pub configuartion_version: u8,
        pub profile_indication: u8,
        pub profile_compatibility: u8,
        pub level_indication: u8,
        pub length_size_minus_one: PaddedByte<6, 1>,
        pub sps: Mp4Array<PaddedByte<3, 1>, Mp4Array<u16, u8>>,
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

impl Default for AvcC {
    fn default() -> Self {
        Self { avc_config: Default::default() }
    }
}

#[cfg(test)]
mod test {
    use crate::bytes_read::Mp4Readable;
    use crate::error::MP4Error;
    use crate::header::BoxHeader;
    use crate::mp4box::avcc::AvcCBox;
    use crate::mp4box::box_trait::{BoxRead, BoxWrite, IBox};

    #[test]
    pub fn test_rebuild() -> Result<(), MP4Error> {
        type Box = AvcCBox;
        futures::executor::block_on(async {
            let base = Box::default();
            let mut buf = vec![];
            let mut cursor = std::io::Cursor::new(&mut buf);
            let pos = base.write(&mut cursor)?;
            assert_eq!(pos, base.byte_size());
            assert_eq!(pos as u64, cursor.position());
            let mut cursor = futures::io::Cursor::new(&mut buf);
            let header = BoxHeader::read(&mut cursor).await?;
            assert_eq!(header.id, Box::ID);
            let new = Box::read(header, &mut cursor).await?;
            assert_eq!(base, new);
            Ok(())
        })
    }

}
