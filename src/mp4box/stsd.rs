use crate::full_box;
use crate::mp4box::box_unknown::UnknownBox;
use crate::types::array::Mp4Array;
use async_trait::async_trait;
use crate::bytes_read::{Mp4Readable, ReadMp4};
use crate::bytes_write::{Mp4Writable, WriteMp4};
use crate::error::MP4Error;
use crate::header::BoxHeader;
use crate::mp4box::avc1::{Avc1Box};
use crate::mp4box::box_trait::{BoxRead, BoxWrite, IBox};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SampleEntry {
    Avc1(Avc1Box),
    Unknown(UnknownBox),
}

#[async_trait]
impl Mp4Readable for SampleEntry {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        let header: BoxHeader = reader.read().await?;
        Ok(match header.id {
            Avc1Box::ID => Self::Avc1(<Avc1Box as BoxRead<_>>::read(header, reader).await?),
            _ => Self::Unknown(<UnknownBox as BoxRead<_>>::read(header, reader).await?)
        })
    }
}

#[async_trait]
impl Mp4Writable for SampleEntry {
    fn byte_size(&self) -> usize {
        match self {
            SampleEntry::Avc1(it) => it.byte_size(),
            SampleEntry::Unknown(it) => it.byte_size()
        }
    }

    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        match self {
            SampleEntry::Avc1(it) => it.write(writer).await,
            SampleEntry::Unknown(it) => it.write(writer).await
        }
    }
}

full_box! {
    box (b"stsd", Stsd, StsdBox, u32) data {
        entries: Mp4Array<u32, SampleEntry>
    }
}
