use fixed::types::{I16F16, I2F30};
use fixed_macro::fixed;
use crate::bytes_read::ReadMp4;
use crate::error::MP4Error;
use crate::bytes_write::WriteMp4;

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
         w: fixed!(1: I2F30)
      }
   }

   pub async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
      Ok(Self {
         a: I16F16::from_bits(reader.read_i32().await?),
         b: I16F16::from_bits(reader.read_i32().await?),
         u: I2F30::from_bits(reader.read_i32().await?),
         c: I16F16::from_bits(reader.read_i32().await?),
         d: I16F16::from_bits(reader.read_i32().await?),
         v: I2F30::from_bits(reader.read_i32().await?),
         x: I16F16::from_bits(reader.read_i32().await?),
         y: I16F16::from_bits(reader.read_i32().await?),
         w: I2F30::from_bits(reader.read_i32().await?)
      })
   }

   pub async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
      let mut count = 0;
      count += writer.write_i32(self.a.to_bits()).await?;
      count += writer.write_i32(self.b.to_bits()).await?;
      count += writer.write_i32(self.u.to_bits()).await?;
      count += writer.write_i32(self.c.to_bits()).await?;
      count += writer.write_i32(self.d.to_bits()).await?;
      count += writer.write_i32(self.v.to_bits()).await?;
      count += writer.write_i32(self.x.to_bits()).await?;
      count += writer.write_i32(self.y.to_bits()).await?;
      count += writer.write_i32(self.w.to_bits()).await?;
      Ok(count)
   }
}

impl Default for MP4Matrix {
   fn default() -> Self {
      Self::unity()
   }
}
