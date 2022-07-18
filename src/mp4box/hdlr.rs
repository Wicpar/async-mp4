use std::mem::size_of;
use std::mem::size_of_val;
use crate::id::BoxId;
use crate::mp4box::box_full::{FullBox, FullBoxData, FullBoxInfo};
use crate::mp4box::box_root::MP4Box;
use crate::r#type::BoxType;
use async_trait::async_trait;
use futures::AsyncReadExt;
use crate::bytes_read::ReadMp4;
use crate::bytes_write::WriteMp4;
use crate::error::MP4Error;
use crate::mp4box::box_trait::{PartialBox, PartialBoxRead, PartialBoxWrite};


pub type HdlrBox = MP4Box<FullBox<Hdlr>>;

#[derive(Debug, Clone)]
pub struct Hdlr {
    handler_type: [u8; 4],
    name: String,
}


impl FullBoxInfo for Hdlr {}

impl PartialBox for Hdlr {
    type ParentData = FullBoxData;

    fn byte_size(&self) -> usize {
        let mut count = 0;
        count += size_of::<u32>(); // reserved
        count += size_of_val(&self.handler_type);
        count += size_of::<[u32; 3]>(); // reserved
        count += self.name.len() + 1;
        count
    }

    const ID: BoxType = BoxType::Id(BoxId(*b"hdlr"));
}

#[async_trait]
impl<R: ReadMp4> PartialBoxRead<R> for Hdlr {
    async fn read_data(_: Self::ParentData, reader: &mut R) -> Result<Self, MP4Error> {
        reader.reserve::<u32>().await?;
        let handler_type = reader.read().await?;
        reader.reserve::<[u32; 3]>().await?;
        let mut name = vec![];
        loop {
            match reader.read().await? {
                0 => break,
                it => name.push(it)
            }
        }
        let name = String::from_utf8(name)?;
        Ok(Self {
            handler_type,
            name,
        })
    }
}

#[async_trait]
impl<W: WriteMp4> PartialBoxWrite<W> for Hdlr {
    async fn write_data(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        count += writer.reserved(size_of::<u32>()).await?;
        count += writer.write_all(&self.handler_type).await?;
        count += writer.reserved(size_of::<[u32; 3]>()).await?;
        count += writer.write_all(self.name.as_bytes()).await?;
        count += writer.write_u8(0).await?; // reserved
        Ok(count)
    }
}
