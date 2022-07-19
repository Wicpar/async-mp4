use fixed::types::{I16F16, I2F30};
use fixed_macro::fixed;
use crate::bytes_read::{Mp4Readable, ReadMp4};
use crate::error::MP4Error;
use crate::bytes_write::{Mp4Writable, WriteMp4};
use async_trait::async_trait;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MP4Matrix {
    pub a: I16F16,
    pub b: I16F16,
    pub u: I2F30,
    pub c: I16F16,
    pub d: I16F16,
    pub v: I2F30,
    pub x: I16F16,
    pub y: I16F16,
    pub w: I2F30,
}

impl MP4Matrix {
    pub fn byte_size() -> usize {
        4 * 9
    }

    pub fn unity() -> MP4Matrix {
        Self {
            a: fixed!(1: I16F16),
            b: fixed!(0: I16F16),
            u: fixed!(0: I2F30),
            c: fixed!(0: I16F16),
            d: fixed!(1: I16F16),
            v: fixed!(0: I2F30),
            x: fixed!(0: I16F16),
            y: fixed!(0: I16F16),
            w: fixed!(1: I2F30),
        }
    }
}

#[async_trait]
impl Mp4Readable for MP4Matrix {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        Ok(Self {
            a: I16F16::from_bits(reader.read().await?),
            b: I16F16::from_bits(reader.read().await?),
            u: I2F30::from_bits(reader.read().await?),
            c: I16F16::from_bits(reader.read().await?),
            d: I16F16::from_bits(reader.read().await?),
            v: I2F30::from_bits(reader.read().await?),
            x: I16F16::from_bits(reader.read().await?),
            y: I16F16::from_bits(reader.read().await?),
            w: I2F30::from_bits(reader.read().await?),
        })
    }
}

#[async_trait]
impl Mp4Writable for MP4Matrix {
    fn byte_size(&self) -> usize {
        let mut count = 0;
        count += self.a.to_bits().byte_size();
        count += self.b.to_bits().byte_size();
        count += self.u.to_bits().byte_size();
        count += self.c.to_bits().byte_size();
        count += self.d.to_bits().byte_size();
        count += self.v.to_bits().byte_size();
        count += self.x.to_bits().byte_size();
        count += self.y.to_bits().byte_size();
        count += self.w.to_bits().byte_size();
        count
    }

    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        count += self.a.to_bits().write(writer).await?;
        count += self.b.to_bits().write(writer).await?;
        count += self.u.to_bits().write(writer).await?;
        count += self.c.to_bits().write(writer).await?;
        count += self.d.to_bits().write(writer).await?;
        count += self.v.to_bits().write(writer).await?;
        count += self.x.to_bits().write(writer).await?;
        count += self.y.to_bits().write(writer).await?;
        count += self.w.to_bits().write(writer).await?;
        Ok(count)
    }
}

impl Default for MP4Matrix {
    fn default() -> Self {
        Self::unity()
    }
}
