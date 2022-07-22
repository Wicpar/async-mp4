use crate::header::BoxHeader;
use async_trait::async_trait;
use futures::{AsyncReadExt};
use crate::bytes_read::ReadMp4;
use crate::bytes_write::{Mp4Writable, WriteMp4};
use crate::error::MP4Error;
use crate::id::BoxId;
use crate::mp4box::box_trait::{BoxRead, BoxWrite, IBox};
use crate::r#type::BoxType;
use crate::size::BoxSize;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct UnknownBox
{
    pub id: BoxType,
    pub data: Vec<u8>
}

impl UnknownBox {
    fn header(&self) -> BoxHeader {
        BoxHeader::from_id_and_inner_size(self.id, self.data.byte_size())
    }
}

impl BoxWrite for UnknownBox
{
    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        count += self.header().write(writer)?;
        count += self.data.write(writer)?;
        Ok(count)
    }
}

impl IBox for UnknownBox
{
    fn byte_size(&self) -> usize {
        self.header().byte_size() + self.data.byte_size()
    }

    const ID: BoxType = BoxType::Id(BoxId(*b"    "));
}

#[async_trait]
impl BoxRead for UnknownBox
{
    async fn read<R: ReadMp4>(header: BoxHeader, reader: &mut R) -> Result<Self, MP4Error> {
        let data = match header.size_minus_self() {
            BoxSize::Known(size) => {
                let mut vec = vec![0u8; size];
                reader.read_exact(vec.as_mut_slice()).await?;
                vec
            }
            BoxSize::Unknown => {
                let mut vec = vec![];
                reader.read_to_end(&mut vec).await?;
                vec
            }
        };
        Ok(Self { id: header.id, data })
    }
}
