use crate::{base_box};
use crate::mp4box::avcc::AvcCBox;
use crate::types::sample::VisualSampleEntry;

base_box! {
    box (b"avc1", Avc1, Avc1Box) data {
        visual_sample_entry: VisualSampleEntry
    } children {
        avcc: AvcCBox
    }
}

impl Default for Avc1 {
    fn default() -> Self {
        Self {
            visual_sample_entry: Default::default(),
            avcc: Some(Default::default())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::bytes_read::Mp4Readable;
    use crate::error::MP4Error;
    use crate::header::BoxHeader;
    use crate::mp4box::avc1::{Avc1Box};
    use crate::mp4box::box_trait::{BoxRead, BoxWrite, IBox};

    #[test]
    pub fn test_rebuild() -> Result<(), MP4Error> {
        type Box = Avc1Box;
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
