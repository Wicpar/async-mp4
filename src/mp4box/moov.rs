use crate::error::MP4Error;
use crate::header::BoxHeader;
use crate::r#box::mvhd::{MvhdBox};
use crate::r#box::{BoxRead, BoxWrite, IBox, PartialBox, PartialBoxRead, PartialBoxWrite};
use crate::r#type::BoxType;
use async_trait::async_trait;
use futures::{AsyncRead, AsyncSeek, AsyncWrite};
use crate::id::BoxId;
use crate::r#box::mvex::MvexBox;
use crate::r#box::r#box::MP4Box;

pub type MoovBox = MP4Box<Moov>;

pub const MOOV: [u8;4] = *b"moov";

#[derive(Debug, Clone, Default)]
pub struct Moov {
    pub mvhd: Option<MvhdBox>,
    pub mvex: Option<MvexBox>,
}


impl PartialBox for Moov {
    type ParentData = ();

    fn byte_size(&self) -> usize {
        self.mvhd.as_ref().map(IBox::byte_size).unwrap_or(0) +
        self.mvex.as_ref().map(IBox::byte_size).unwrap_or(0)
    }

    fn id() -> BoxType {
        MOOV.into()
    }
}

#[async_trait]
impl<R> PartialBoxRead<R> for Moov
    where
        R: AsyncRead + AsyncSeek + Unpin + Send + Sync {
    async fn read_data(_: Self::ParentData, _: &mut R) -> Result<Self, MP4Error> {
        Ok(Default::default())
    }

    async fn read_child(&mut self, header: BoxHeader, reader: &mut R) -> Result<(), MP4Error> {
        match header.id {
            // god damn rust not allowing b"mvhd"...
            BoxType::Id(BoxId(crate::r#box::mvhd::MVHD)) => self.mvhd = Some(MvhdBox::read(header, reader).await?),
            BoxType::Id(BoxId(crate::r#box::mvex::MVEX)) => self.mvex = Some(MvexBox::read(header, reader).await?),
            _ => {}
        }
        Ok(())
    }
}

#[async_trait]
impl<W> PartialBoxWrite<W> for Moov
    where
        W: AsyncWrite + Unpin + Send + Sync {
    async fn write_children(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        if let Some(mvhd) = &self.mvhd { count += mvhd.write(writer).await?; }
        if let Some(mvex) = &self.mvex { count += mvex.write(writer).await?; }
        Ok(count)
    }
}
