
use async_trait::async_trait;
use crate::{default_flags, full_box};
use crate::bytes_read::{Mp4Readable, ReadMp4};
use crate::bytes_write::{Mp4Writable, WriteMp4};
use crate::error::MP4Error;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VideoGraphicsMode(pub u16);

impl VideoGraphicsMode {
    pub const COPY: VideoGraphicsMode = VideoGraphicsMode(0);
}

impl Mp4Writable for VideoGraphicsMode {
    fn byte_size(&self) -> usize {
        self.0.byte_size()
    }

    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        self.0.write(writer)
    }
}
#[async_trait]
impl Mp4Readable for VideoGraphicsMode {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        Ok(Self(u16::read(reader).await?))
    }
}

default_flags!(VmhdFlags, 1);

full_box! {
    box (b"vmhd", Vmhd, VmhdBox, VmhdFlags)
    data {
        mode: VideoGraphicsMode,
        color: [u16; 3]
    }
}

impl Default for Vmhd {
    fn default() -> Self {
        Self {
            mode: VideoGraphicsMode::COPY,
            color: Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::bytes_read::Mp4Readable;
    use crate::error::MP4Error;
    use crate::header::BoxHeader;
    use crate::mp4box::box_trait::{BoxRead, BoxWrite, PartialBox};
    use crate::mp4box::vmhd::{Vmhd, VmhdBox};

    #[test]
    pub fn test_rebuild() -> Result<(), MP4Error> {
        futures::executor::block_on(async {
            let base = VmhdBox::default();
            let mut buf = vec![];
            let mut cursor = std::io::Cursor::new(&mut buf);
            let pos = base.write(&mut cursor)?;
            assert_eq!(pos as u64, cursor.position());
            let mut cursor = futures::io::Cursor::new(&mut buf);
            let header = BoxHeader::read(&mut cursor).await?;
            assert_eq!(header.id, Vmhd::ID);
            let new = VmhdBox::read(header, &mut cursor).await?;
            assert_eq!(base, new);
            Ok(())
        })
    }

}
