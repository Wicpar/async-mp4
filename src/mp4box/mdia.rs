use async_trait::async_trait;

use crate::bytes_read::ReadMp4;
use crate::bytes_write::WriteMp4;
use crate::error::MP4Error;
use crate::header::BoxHeader;
use crate::id::BoxId;
use crate::mp4box::{IBox, PartialBox, PartialBoxRead, PartialBoxWrite, BoxWrite, BoxRead};
use crate::mp4box::mdhd::MdhdBox;
use crate::mp4box::rootbox::MP4Box;
use crate::r#type::BoxType;

pub type MdiaBox = MP4Box<Mdia>;

#[derive(Debug, Clone, Default)]
pub struct Mdia {
    pub mdhd: Option<MdhdBox>
}

impl PartialBox for Mdia {
    type ParentData = ();

    fn byte_size(&self) -> usize {
        self.mdhd.as_ref().map(IBox::byte_size).unwrap_or(0)
    }

    const ID: BoxType = BoxType::Id(BoxId(*b"mdia"));
}

#[async_trait]
impl<R: ReadMp4> PartialBoxRead<R> for Mdia {
    async fn read_data(_: Self::ParentData, _: &mut R) -> Result<Self, MP4Error> {
        Ok(Default::default())
    }

    async fn read_child(&mut self, header: BoxHeader, reader: &mut R) -> Result<(), MP4Error> {
        match header.id {
            MdhdBox::ID => self.mdhd = Some(MdhdBox::read(header, reader).await?),
            _ => {}
        }
        Ok(())
    }
}

#[async_trait]
impl<W: WriteMp4> PartialBoxWrite<W> for Mdia {
    async fn write_children(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        if let Some(mdhd) = &self.mdhd { count += mdhd.write(writer).await?; }
        Ok(count)
    }
}