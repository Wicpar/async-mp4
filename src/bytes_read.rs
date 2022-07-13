use std::io::SeekFrom;

use async_trait::async_trait;
use byteorder_async::{BigEndian};
use futures::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt};
use byteorder_async::ReaderToByteOrder;

use crate::error::MP4Error;

#[async_trait]
pub trait ReadMp4: AsyncRead + AsyncSeek + Unpin + Send + Sync {
    #[inline]
    async fn read_u8(&mut self) -> Result<u8, MP4Error> {
        Ok(self.byte_order().read_u8().await?)
    }
    #[inline]
    async fn read_u16(&mut self) -> Result<u16, MP4Error> {
        Ok(self.byte_order().read_u16::<BigEndian>().await?)
    }
    #[inline]
    async fn read_u24(&mut self) -> Result<u32, MP4Error> {
        Ok(self.byte_order().read_u24::<BigEndian>().await?)
    }
    #[inline]
    async fn read_u32(&mut self) -> Result<u32, MP4Error> {
        Ok(self.byte_order().read_u32::<BigEndian>().await?)
    }
    #[inline]
    async fn read_u64(&mut self) -> Result<u64, MP4Error> {
        Ok(self.byte_order().read_u64::<BigEndian>().await?)
    }

    #[inline]
    async fn read_i8(&mut self) -> Result<i8, MP4Error> {
        Ok(self.byte_order().read_i8().await?)
    }
    #[inline]
    async fn read_i16(&mut self) -> Result<i16, MP4Error> {
        Ok(self.byte_order().read_i16::<BigEndian>().await?)
    }
    #[inline]
    async fn read_i24(&mut self) -> Result<i32, MP4Error> {
        Ok(self.byte_order().read_i24::<BigEndian>().await?)
    }
    #[inline]
    async fn read_i32(&mut self) -> Result<i32, MP4Error> {
        Ok(self.byte_order().read_i32::<BigEndian>().await?)
    }
    #[inline]
    async fn read_i64(&mut self) -> Result<i64, MP4Error> {
        Ok(self.byte_order().read_i64::<BigEndian>().await?)
    }

    #[inline]
    async fn read(&mut self, buf: &mut [u8]) -> Result<(), MP4Error> {
        Ok(self.read_exact(buf).await?)
    }

    #[inline]
    async fn reserved(&mut self, count: usize) -> Result<(), MP4Error> {
        self.seek(SeekFrom::Current(count as i64)).await?;
        Ok(())
    }
}

impl<T: AsyncRead + AsyncSeek + Unpin + Send + Sync> ReadMp4 for T {}
