use std::fmt::{Display, Formatter};
use std::io::{Seek, SeekFrom};
use std::mem;
use byteorder_async::{BigEndian, ReaderToByteOrder, WriterToByteOrder};
use futures::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt, AsyncWrite, AsyncWriteExt};
use uuid::Uuid;
use crate::error::MalformedBoxError::UnknownSizeForUnknownBox;
use crate::error::MP4Error;
use crate::id::BoxId;
use crate::r#type::BoxType;
use crate::size::BoxSize;
use crate::size::BoxSize::Known;

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
            BoxSize::Known(size) => BoxSize::Known(self.byte_size() - size),
            BoxSize::Unknown => BoxSize::Unknown
        }
    }

    pub async fn read<R: AsyncRead + Unpin>(reader: &mut R) -> Result<BoxHeader, MP4Error> {
        let size = reader.byte_order().read_u32::<BigEndian>().await?;
        let id = BoxId::read(reader).await?;
        let size = match size {
            0 => BoxSize::Unknown,
            1 =>  {
                let size = reader.byte_order().read_u64::<BigEndian>().await?;
                BoxSize::Known(size as _)
            }
            _ => BoxSize::Known(size as _)
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

    pub async fn write<W: AsyncWrite + Unpin>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let (id, uuid) = match self.id {
            BoxType::Id(id) => (id, None),
            BoxType::UUID(uuid) => (BoxId(*b"uuid"), Some(uuid))
        };
        let (size, big_size) = match self.size {
            BoxSize::Known(size) => {
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
            BoxSize::Unknown => (0, None)
        };
        let mut count = 0;
        writer.byte_order().write_u32::<BigEndian>(size).await?;
        count += mem::size_of::<u32>();
        count += id.write(writer).await?;
        if let Some(size) = big_size {
            writer.byte_order().write_u64::<BigEndian>(size).await?;
            count += mem::size_of::<u64>();
        }
        if let Some(uuid) = uuid {
            writer.write_all(&uuid.into_bytes()).await?;
            count += mem::size_of::<uuid::Bytes>();
        }
        Ok(count)
    }
}
