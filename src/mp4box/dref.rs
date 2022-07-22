use crate::{full_box};
use crate::bytes_read::{Mp4Readable, ReadMp4};
use crate::error::{MP4Error};
use crate::header::BoxHeader;
use crate::mp4box::box_trait::{BoxRead, BoxWrite, IBox};
use crate::mp4box::url::UrlBox;
use crate::mp4box::urn::UrnBox;
use crate::types::array::Mp4Array;
use async_trait::async_trait;
use DataEntryBox::{Unknown, Urn};
use crate::bytes_write::{Mp4Writable, WriteMp4};
use crate::mp4box::box_unknown::UnknownBox;
use crate::mp4box::dref::DataEntryBox::Url;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DataEntryBox {
    Url(UrlBox),
    Urn(UrnBox),
    Unknown(UnknownBox)
}

#[async_trait]
impl Mp4Readable for DataEntryBox {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        let header: BoxHeader = reader.read().await?;
        Ok(match header.id {
            UrlBox::ID => Url(<UrlBox as BoxRead>::read(header, reader).await?),
            UrnBox::ID => Urn(<UrnBox as BoxRead>::read(header, reader).await?),
            _ => Unknown(<UnknownBox as BoxRead>::read(header, reader).await?)
        })
    }
}

impl Mp4Writable for DataEntryBox {
    fn byte_size(&self) -> usize {
        match self {
            Url(it) =>  it.byte_size(),
            Urn(it ) => it.byte_size(),
            Unknown(it) => it.byte_size()
        }
    }

    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        match self {
            Url(it) => it.write(writer),
            Urn(it ) => it.write(writer),
            Unknown(it) => it.write(writer),
        }
    }
}

full_box! {
    box (b"dref", Dref, DrefBox, u32) data {
        entries: Mp4Array<u32, DataEntryBox>
    }
}

impl Default for Dref {
    fn default() -> Self {
        Self {
            entries: vec![Url(UrlBox::default())].into()
        }
    }
}
