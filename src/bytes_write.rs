use std::ops::BitOr;
use futures::{AsyncWrite, AsyncWriteExt};
use async_trait::async_trait;
use byteorder_async::{BigEndian, WriterToByteOrder};
use crate::bytes_reserve::{Mp4Reservable, Mp4Reserve};
use crate::error::MP4Error;

#[async_trait]
pub trait Mp4Writable {
    fn byte_size(&self) -> usize;
    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error>;
}

#[async_trait]
pub trait Mp4VersionedWritable<F: Default + Send + Sync> {
    fn required_version(&self) -> u8 {0}
    fn required_flags(&self) -> F {F::default()}
    fn versioned_byte_size(&self, version: u8, flags: F) -> usize;
    async fn versioned_write<W: WriteMp4>(&self, version: u8, flags: F, writer: &mut W) -> Result<usize, MP4Error>;
}

#[async_trait]
impl<F: Default + Send + Sync, T: Mp4Writable + Send + Sync> Mp4VersionedWritable<F> for  T {

    fn versioned_byte_size(&self, _: u8, _: F) -> usize {
        self.byte_size()
    }

    async fn versioned_write<W: WriteMp4>(&self, _: u8, _: F, writer: &mut W) -> Result<usize, MP4Error> {
        self.write(writer).await
    }
}

#[async_trait]
impl Mp4Writable for u8 {
    fn byte_size(&self) -> usize {
        1
    }

    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        writer.byte_order().write_u8(*self).await?;
        Ok(self.byte_size())
    }
}

#[async_trait]
impl Mp4Writable for u16 {
    fn byte_size(&self) -> usize {
        2
    }
    async fn write<W: WriteMp4 + ?Sized>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        writer.byte_order().write_u16::<BigEndian>(*self).await?;
        Ok(self.byte_size())
    }
}

#[async_trait]
impl Mp4Writable for u32 {
    fn byte_size(&self) -> usize {
        4
    }
    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        writer.byte_order().write_u32::<BigEndian>(*self).await?;
        Ok(self.byte_size())
    }
}

#[async_trait]
impl Mp4Writable for u64 {
    fn byte_size(&self) -> usize {
        8
    }
    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        writer.byte_order().write_u64::<BigEndian>(*self).await?;
        Ok(self.byte_size())
    }
}

#[async_trait]
impl Mp4Writable for i8 {
    fn byte_size(&self) -> usize {
        1
    }
    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        writer.byte_order().write_i8(*self).await?;
        Ok(self.byte_size())
    }
}

#[async_trait]
impl Mp4Writable for i16 {
    fn byte_size(&self) -> usize {
        2
    }
    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        writer.byte_order().write_i16::<BigEndian>(*self).await?;
        Ok(self.byte_size())
    }
}

#[async_trait]
impl Mp4Writable for i32 {
    fn byte_size(&self) -> usize {
        4
    }
    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        writer.byte_order().write_i32::<BigEndian>(*self).await?;
        Ok(self.byte_size())
    }
}

#[async_trait]
impl Mp4Writable for i64 {
    fn byte_size(&self) -> usize {
        8
    }
    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        writer.byte_order().write_i64::<BigEndian>(*self).await?;
        Ok(self.byte_size())
    }
}

#[async_trait]
impl<T: Mp4Writable + Send + Sync> Mp4Writable for [T] {
    fn byte_size(&self) -> usize {
        self.iter().map(Mp4Writable::byte_size).sum()
    }

    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        for elem in self {
            count += elem.write(writer).await?;
        }
        Ok(count)
    }
}

#[async_trait]
impl<T: Mp4Writable  + Send + Sync, const N: usize> Mp4Writable for [T; N] {
    fn byte_size(&self) -> usize {
        self.iter().map(Mp4Writable::byte_size).sum()
    }
    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        for elem in self {
            count += elem.write(writer).await?;
        }
        Ok(count)
    }
}

#[async_trait]
impl <T: Mp4Writable + Send + Sync> Mp4Writable for Option<T> {
    fn byte_size(&self) -> usize {
        self.iter().map(Mp4Writable::byte_size).sum()
    }
    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        if let Some(elem) = self {
            elem.write(writer).await
        } else {
            Ok(0)
        }
    }
}

#[async_trait]
impl <T: Mp4Writable + Send + Sync> Mp4Writable for Vec<T> {
    fn byte_size(&self) -> usize {
        self.iter().map(Mp4Writable::byte_size).sum()
    }
    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        for elem in self {
            count += elem.write(writer).await?;
        }
        Ok(count)
    }
}

#[async_trait]
pub trait WriteMp4: AsyncWrite + Unpin + Send + Sync + Sized {

    #[inline]
    async fn write_u24(&mut self, n: u32) -> Result<usize, MP4Error> {
        self.byte_order().write_u24::<BigEndian>(n).await?;
        Ok(3)
    }

    #[inline]
    async fn write_i24(&mut self, n: i32) -> Result<usize, MP4Error> {
        self.byte_order().write_i24::<BigEndian>(n).await?;
        Ok(3)
    }

    #[inline]
    async fn reserve<T: Mp4Reservable>(&mut self) -> Result<usize, MP4Error> {
        let buf = [0u8, T::BYTE_SIZE];
        self.write_all(&buf).await?;
        Ok(T::BYTE_SIZE)
    }
}

impl<T: AsyncWrite + Unpin + Send + Sync> WriteMp4 for T {}
