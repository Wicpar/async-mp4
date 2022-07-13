use crate::mp4box::{BoxRead, BoxWrite, IBox, PartialBox, PartialBoxRead, PartialBoxWrite};
use crate::mp4box::rootbox::MP4Box;
use async_trait::async_trait;
use crate::bytes_read::ReadMp4;
use crate::bytes_write::WriteMp4;
use crate::error::MP4Error;
use crate::header::BoxHeader;
use crate::id::BoxId;
use crate::mp4box::mdia::MdiaBox;
use crate::mp4box::tkhd::TkhdBox;
use crate::r#type::BoxType;

pub type TrakBox = MP4Box<Trak>;

#[derive(Debug, Clone, Default)]
pub struct Trak {
    pub tkhd: Option<TkhdBox>,
    pub mdia: Option<MdiaBox>,
}

impl PartialBox for Trak {
    type ParentData = ();

    fn byte_size(&self) -> usize {
        self.tkhd.as_ref().map(IBox::byte_size).unwrap_or(0)+
        self.mdia.as_ref().map(IBox::byte_size).unwrap_or(0)
    }

    const ID: BoxType = BoxType::Id(BoxId(*b"trak"));
}

#[async_trait]
impl<R: ReadMp4> PartialBoxRead<R> for Trak {
    async fn read_data(_: Self::ParentData, _: &mut R) -> Result<Self, MP4Error> {
        Ok(Default::default())
    }

    async fn read_child(&mut self, header: BoxHeader, reader: &mut R) -> Result<(), MP4Error> {
        match header.id {
            TkhdBox::ID => self.tkhd = Some(TkhdBox::read(header, reader).await?),
            MdiaBox::ID => self.mdia = Some(MdiaBox::read(header, reader).await?),
            _ => {}
        }
        Ok(())
    }
}

#[async_trait]
impl<W: WriteMp4> PartialBoxWrite<W> for Trak {
    async fn write_children(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        if let Some(tkhd) = &self.tkhd { count += tkhd.write(writer).await?; }
        if let Some(mdia) = &self.mdia { count += mdia.write(writer).await?; }
        Ok(count)
    }
}
