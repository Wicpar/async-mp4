
use uuid::Uuid;
use crate::bytes_read::{Mp4Readable, ReadMp4};
use crate::bytes_write::{Mp4Writable, WriteMp4};
use crate::error::MP4Error;
use crate::id::BoxId;
use crate::r#type::BoxType;
use crate::size::BoxSize;
use crate::size::BoxSize::Known;
use crate::size::BoxSize::Unknown;
use async_trait::async_trait;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct BoxHeader {
    pub size: BoxSize,
    pub id: BoxType
}

impl BoxHeader {

    pub fn from_id_and_inner_size(id: BoxType, inner_size: usize) -> Self {
        Self {
            size: BoxSize::from_size_without_self(inner_size + id.byte_size()),
            id
        }
    }

    pub fn size_minus_self(&self) -> BoxSize {
        match self.size {
            Known(size) => Known(self.byte_size() - size),
            Unknown => Unknown
        }
    }

}

#[async_trait]
impl Mp4Readable for BoxHeader {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        let size: u32 = reader.read().await?;
        let id = BoxId::read(reader).await?;
        let size = match size {
            0 => Unknown,
            1 =>  {
                let size: u64 = reader.read().await?;
                Known(size as _)
            }
            _ => Known(size as _)
        };
        let id = if id == b"uuid" {
            BoxType::UUID(reader.read().await?)
        } else {
            BoxType::Id(id)
        };
        Ok(Self {
            size, id
        })
    }
}

#[async_trait]
impl Mp4Writable for BoxHeader {
    fn byte_size(&self) -> usize {
        self.size.byte_size() + self.id.byte_size()
    }


    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let (id, uuid) = match self.id {
            BoxType::Id(id) => (id, None),
            BoxType::UUID(uuid) => (BoxId(*b"uuid"), Some(uuid))
        };
        let (size, big_size) = match self.size {
            Known(size) => {
                if cfg!(target_pointer_width = "64") {
                    if size > u32::MAX as usize {
                        (1, Some(size as u64))
                    } else {
                        (size as u32, None)
                    }
                } else {
                    (size as u32, None)
                }
            }
            Unknown => (0, None)
        };
        let mut count = 0;
        count += size.write(writer).await?;
        count += id.write(writer).await?;
        count += big_size.write(writer).await?;
        count += uuid.write(writer).await?;
        Ok(count)
    }
}

#[async_trait]
impl Mp4Readable for Uuid {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        Ok(Uuid::from_bytes(reader.read().await?))
    }
}

#[async_trait]
impl Mp4Writable for Uuid {
    fn byte_size(&self) -> usize {
        self.as_bytes().byte_size()
    }

    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        self.as_bytes().write(writer).await
    }
}
