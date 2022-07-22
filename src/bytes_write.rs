use std::io::Write;
use std::ops::BitOr;
use byteorder::{BigEndian, WriteBytesExt};
use crate::error::MP4Error;

pub trait FlagTrait: Copy + Default + BitOr<Output=Self> + Into<u32> + From<u32> + Send + Sync + 'static {}
impl<T: Copy + Default + BitOr<Output=Self> + Into<u32> + From<u32> + Send + Sync + 'static> FlagTrait for T {}

pub trait Mp4Writable {
    fn byte_size(&self) -> usize;
    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error>;
}

pub trait Mp4VersionedWritable<F: FlagTrait> {
    fn required_version(&self) -> u8 {0}
    fn required_flags(&self) -> F {F::default()}
    fn versioned_byte_size(&self, version: u8, flags: F) -> usize;
    fn versioned_write<W: WriteMp4>(&self, version: u8, flags: F, writer: &mut W) -> Result<usize, MP4Error>;
}

impl<F: FlagTrait, T: Mp4Writable + Send + Sync> Mp4VersionedWritable<F> for  T {
    fn versioned_byte_size(&self, _: u8, _: F) -> usize {
        self.byte_size()
    }
    fn versioned_write<W: WriteMp4>(&self, _: u8, _: F, writer: &mut W) -> Result<usize, MP4Error> {
        self.write(writer)
    }
}

impl Mp4Writable for u8 {
    fn byte_size(&self) -> usize {
        1
    }

    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        writer.write_u8(*self)?;
        Ok(self.byte_size())
    }
}

impl Mp4Writable for u16 {
    fn byte_size(&self) -> usize {
        2
    }
    fn write<W: WriteMp4 + ?Sized>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        writer.write_u16::<BigEndian>(*self)?;
        Ok(self.byte_size())
    }
}

impl Mp4Writable for u32 {
    fn byte_size(&self) -> usize {
        4
    }
    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        writer.write_u32::<BigEndian>(*self)?;
        Ok(self.byte_size())
    }
}

impl Mp4Writable for u64 {
    fn byte_size(&self) -> usize {
        8
    }
    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        writer.write_u64::<BigEndian>(*self)?;
        Ok(self.byte_size())
    }
}

impl Mp4Writable for i8 {
    fn byte_size(&self) -> usize {
        1
    }
    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        writer.write_i8(*self)?;
        Ok(self.byte_size())
    }
}

impl Mp4Writable for i16 {
    fn byte_size(&self) -> usize {
        2
    }
    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        writer.write_i16::<BigEndian>(*self)?;
        Ok(self.byte_size())
    }
}

impl Mp4Writable for i32 {
    fn byte_size(&self) -> usize {
        4
    }
    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        writer.write_i32::<BigEndian>(*self)?;
        Ok(self.byte_size())
    }
}

impl Mp4Writable for i64 {
    fn byte_size(&self) -> usize {
        8
    }
    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        writer.write_i64::<BigEndian>(*self)?;
        Ok(self.byte_size())
    }
}

impl<T: Mp4Writable + Send + Sync> Mp4Writable for [T] {
    fn byte_size(&self) -> usize {
        self.iter().map(Mp4Writable::byte_size).sum()
    }

    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        for elem in self {
            count += elem.write(writer)?;
        }
        Ok(count)
    }
}

impl<T: Mp4Writable  + Send + Sync, const N: usize> Mp4Writable for [T; N] {
    fn byte_size(&self) -> usize {
        self.as_slice().byte_size()
    }
    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        self.as_slice().write(writer)
    }
}

impl <T: Mp4Writable + Send + Sync> Mp4Writable for Option<T> {
    fn byte_size(&self) -> usize {
        self.iter().map(Mp4Writable::byte_size).sum()
    }
    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        if let Some(elem) = self {
            elem.write(writer)
        } else {
            Ok(0)
        }
    }
}

impl <T: Mp4Writable + Send + Sync> Mp4Writable for Vec<T> {
    fn byte_size(&self) -> usize {
        self.as_slice().byte_size()
    }
    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        self.as_slice().write(writer)
    }
}

pub trait WriteMp4: Write + Unpin + Send + Sync + Sized {

    #[inline]
    fn write_u24(&mut self, n: u32) -> Result<usize, MP4Error> {
       WriteBytesExt::write_u24::<BigEndian>(self, n)?;
        Ok(3)
    }

    #[inline]
    fn write_i24(&mut self, n: i32) -> Result<usize, MP4Error> {
        WriteBytesExt::write_i24::<BigEndian>(self, n)?;
        Ok(3)
    }
}

impl<T: Write + Unpin + Send + Sync> WriteMp4 for T {}
