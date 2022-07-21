use std::mem;
use futures::{AsyncSeek, AsyncSeekExt};
use std::io::SeekFrom;
use crate::error::MP4Error;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BoxSize {
    Known(usize),
    Unknown
}

impl BoxSize {

    pub fn from_size_without_self(size: usize) -> Self {
        Self::Known(size + if cfg!(target_pointer_width = "64") {
            mem::size_of::<u32>() + if size + mem::size_of::<u32>() > u32::MAX as usize {
                mem::size_of::<u64>()
            } else {
                0
            }
        } else {
            mem::size_of::<u32>()
        })
    }

    pub fn byte_size(&self) -> usize {
        match self {
            BoxSize::Known(size) => {
                if cfg!(target_pointer_width = "64") {
                    mem::size_of::<u32>() + if size > &(u32::MAX as usize) {
                        mem::size_of::<u64>()
                    } else {
                        0
                    }
                } else {
                    mem::size_of::<u32>()
                }
            }
            BoxSize::Unknown => mem::size_of::<u32>()
        }
    }

    pub async fn ended<R: AsyncSeek + Unpin>(&self, start: u64, reader: &mut R) -> Result<bool, MP4Error> {
        Ok(match self {
            BoxSize::Known(size) => {
                let size = *size as i64;
                let mut pos = reader.seek(SeekFrom::Current(0)).await?;
                let read = (pos - start) as i64;
                if read > size {
                    pos = reader.seek(SeekFrom::Current(size - read)).await?;
                }
                read >= size
            }
            BoxSize::Unknown => false
        })
    }
}
