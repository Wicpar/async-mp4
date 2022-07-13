use crate::mp4box::full_box::FullBox;
use crate::mp4box::{PartialBox, PartialBoxRead, PartialBoxWrite};
use crate::mp4box::rootbox::MP4Box;
use crate::r#type::BoxType;
use async_trait::async_trait;
use futures::{AsyncRead, AsyncSeek, AsyncWrite};
use crate::error::MP4Error;
use crate::header::BoxHeader;
use crate::id::BoxId;

pub type MvexBox = MP4Box<Mvex>;

pub const MVEX: [u8;4] = *b"mvex";

#[derive(Debug, Clone, Default)]
pub struct Mvex {
}


impl PartialBox for Mvex {
    type ParentData = ();

    fn byte_size(&self) -> usize {
        0
    }

    const ID: BoxType = BoxType::Id(BoxId(*b"mvex"));
}

#[async_trait]
impl<R> PartialBoxRead<R> for Mvex
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
impl<W> PartialBoxWrite<W> for Mvex
    where
        W: AsyncWrite + Unpin + Send + Sync {
    async fn write_children(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        Ok(count)
    }
}
