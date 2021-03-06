use std::fmt::Debug;

use async_trait::async_trait;
use byteorder_async::{BigEndian};
use futures::{AsyncRead, AsyncSeek};
use byteorder_async::ReaderToByteOrder;
use crate::bytes_write::FlagTrait;

use crate::error::MP4Error;

#[async_trait]
pub trait Mp4Readable: Sized {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error>;
}

#[async_trait]
pub trait Mp4VersionedReadable<F: FlagTrait>: Sized {
    async fn versioned_read<R: ReadMp4>(version: u8, flags: F, reader: &mut R) -> Result<Self, MP4Error>;
}

#[async_trait]
impl<F: FlagTrait, T: Mp4Readable> Mp4VersionedReadable<F> for  T {
    async fn versioned_read<R: ReadMp4>(_: u8, _: F, reader: &mut R) -> Result<Self, MP4Error> {
        T::read(reader).await
    }
}

#[async_trait]
impl Mp4Readable for u8 {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        Ok(reader.byte_order().read_u8().await?)
    }
}

#[async_trait]
impl Mp4Readable for u16 {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        Ok(reader.byte_order().read_u16::<BigEndian>().await?)
    }
}
#[async_trait]
impl Mp4Readable for u32 {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        Ok(reader.byte_order().read_u32::<BigEndian>().await?)
    }
}
#[async_trait]
impl Mp4Readable for u64 {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        Ok(reader.byte_order().read_u64::<BigEndian>().await?)
    }
}

#[async_trait]
impl Mp4Readable for i8 {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        Ok(reader.byte_order().read_i8().await?)
    }
}

#[async_trait]
impl Mp4Readable for i16 {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        Ok(reader.byte_order().read_i16::<BigEndian>().await?)
    }
}
#[async_trait]
impl Mp4Readable for i32 {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        Ok(reader.byte_order().read_i32::<BigEndian>().await?)
    }
}
#[async_trait]
impl Mp4Readable for i64 {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        Ok(reader.byte_order().read_i64::<BigEndian>().await?)
    }
}

#[async_trait]
impl<const N: usize, T: Mp4Readable + Send + Sync + Debug> Mp4Readable for [T; N] {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        let mut out = vec![];
        for _ in 0..N {
            out.push(T::read(reader).await?);
        }
        Ok(out.try_into().unwrap())
    }
}


#[async_trait]
pub trait ReadMp4: AsyncRead + AsyncSeek + Unpin + Send + Sync + Sized {

    #[inline]
    async fn read_u24(&mut self) -> Result<u32, MP4Error> {
        Ok(self.byte_order().read_u24::<BigEndian>().await?)
    }

    #[inline]
    async fn read_i24(&mut self) -> Result<i32, MP4Error> {
        Ok(self.byte_order().read_i24::<BigEndian>().await?)
    }

    async fn read<T: Mp4Readable>(&mut self) -> Result<T, MP4Error> {
        T::read(self).await
    }

    async fn versioned_read<T: Mp4VersionedReadable<F>, F: FlagTrait>(&mut self, version: u8, flags: F) -> Result<T, MP4Error> { T::versioned_read(version, flags, self).await }
}

impl<T: AsyncRead + AsyncSeek + Unpin + Send + Sync> ReadMp4 for T {}
