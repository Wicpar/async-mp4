
use futures::AsyncReadExt;
use uuid::Uuid;
use crate::bytes_read::ReadMp4;
use crate::bytes_write::WriteMp4;
use crate::error::MP4Error;
use crate::id::BoxId;
use crate::r#type::BoxType;
use crate::size::BoxSize;
use crate::size::BoxSize::Known;
use crate::size::BoxSize::Unknown;

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

    pub fn byte_size(&self) -> usize {
        self.size.byte_size() + self.id.byte_size()
    }

    pub fn size_minus_self(&self) -> BoxSize {
        match self.size {
            Known(size) => Known(self.byte_size() - size),
            Unknown => Unknown
        }
    }

    pub async fn read<R: ReadMp4>(reader: &mut R) -> Result<BoxHeader, MP4Error> {
        let size = reader.read_u32().await?;
        let id = BoxId::read(reader).await?;
        let size = match size {
            0 => Unknown,
            1 =>  {
                let size = reader.read_u64().await?;
                Known(size as _)
            }
            _ => Known(size as _)
        };
        let id = if id == b"uuid" {
            let mut data = [0u8; 16];
            reader.read_exact(&mut data).await?;
            BoxType::UUID(Uuid::from_bytes(data))
        } else {
            BoxType::Id(id)
        };
        Ok(Self {
            size, id
        })
    }

    pub async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
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
        count += writer.write_u32(size).await?;
        count += id.write(writer).await?;
        if let Some(size) = big_size {
            count += writer.write_u64(size).await?;
        }
        if let Some(uuid) = uuid {
            count += writer.write_all(&uuid.into_bytes()).await?;
        }
        Ok(count)
    }
}
