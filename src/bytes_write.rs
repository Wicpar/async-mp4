use futures::{AsyncWrite};
use async_trait::async_trait;
use byteorder_async::{BigEndian, WriterToByteOrder};
use crate::error::MP4Error;

#[async_trait]
pub trait WriteMp4: AsyncWrite + Unpin + Send + Sync {
    #[inline]
    async fn write_u8(&mut self, n: u8) -> Result<usize, MP4Error> {
        self.byte_order().write_u8(n).await?;
        Ok(1)
    }
    #[inline]
    async fn write_u16(&mut self, n: u16) -> Result<usize, MP4Error> {
        self.byte_order().write_u16::<BigEndian>(n).await?;
        Ok(2)
    }
    #[inline]
    async fn write_u24(&mut self, n: u32) -> Result<usize, MP4Error> {
        self.byte_order().write_u24::<BigEndian>(n).await?;
        Ok(3)
    }
    #[inline]
    async fn write_u32(&mut self, n: u32) -> Result<usize, MP4Error> {
        self.byte_order().write_u32::<BigEndian>(n).await?;
        Ok(4)
    }
    #[inline]
    async fn write_u64(&mut self, n: u64) -> Result<usize, MP4Error> {
        self.byte_order().write_u64::<BigEndian>(n).await?;
        Ok(8)
    }

    #[inline]
    async fn write_i8(&mut self, n: i8) -> Result<usize, MP4Error> {
        self.byte_order().write_i8(n).await?;
        Ok(1)
    }
    #[inline]
    async fn write_i16(&mut self, n: i16) -> Result<usize, MP4Error> {
        self.byte_order().write_i16::<BigEndian>(n).await?;
        Ok(2)
    }
    #[inline]
    async fn write_i24(&mut self, n: i32) -> Result<usize, MP4Error> {
        self.byte_order().write_i24::<BigEndian>(n).await?;
        Ok(3)
    }
    #[inline]
    async fn write_i32(&mut self, n: i32) -> Result<usize, MP4Error> {
        self.byte_order().write_i32::<BigEndian>(n).await?;
        Ok(4)
    }
    #[inline]
    async fn write_i64(&mut self, n: i64) -> Result<usize, MP4Error> {
        self.byte_order().write_i64::<BigEndian>(n).await?;
        Ok(8)
    }

    #[inline]
    async fn write_all(&mut self, buf: &[u8]) -> Result<usize, MP4Error> {
        self.byte_order().write_all(buf).await?;
        Ok(buf.len())
    }

    #[inline]
    async fn reserved(&mut self, count: usize) -> Result<usize, MP4Error> {
        let buf = vec![0; count];
        self.write_all(&buf).await?;
        Ok(count)
    }
}

impl<T: AsyncWrite + Unpin + Send + Sync> WriteMp4 for T {}
