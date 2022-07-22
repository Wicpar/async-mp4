use std::fmt::{Debug, Formatter};
use num_traits::AsPrimitive;
use crate::bytes_read::{Mp4Readable, ReadMp4};
use crate::error::MP4Error;
use async_trait::async_trait;
use crate::bytes_write::{Mp4Writable, WriteMp4};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PaddedByte<const N: usize, const V: usize = 0>(u8);

impl<const N: usize, const V: usize> Default for PaddedByte<N, V> {
    fn default() -> Self {
        Self::from(0)
    }
}

impl<const N: usize, const V: usize> AsPrimitive<usize> for PaddedByte<N, V> {
    fn as_(self) -> usize {
        let u: u8 = self.as_();
        u as usize
    }
}

impl<const N: usize, const V: usize> AsPrimitive<PaddedByte<N, V>> for usize  {
    fn as_(self) -> PaddedByte<N, V> {
        let u: u8 = self.as_();
        u.as_()
    }
}

impl<const N: usize, const V: usize> From<u8> for PaddedByte<N, V> {
    fn from(value: u8) -> Self {
        value.as_()
    }
}

impl<const N: usize, const V: usize> From<PaddedByte<N, V>> for u8 {
    fn from(value:  PaddedByte<N, V>) -> Self {
        value.as_()
    }
}

#[async_trait]
impl<const N: usize, const V: usize> Mp4Readable for PaddedByte<N, V> {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        Ok(Self(reader.read::<u8>().await? & Self::MASK | Self::PAD))
    }
}

impl<const N: usize, const V: usize> Mp4Writable for PaddedByte<N, V> {
    fn byte_size(&self) -> usize {
        self.0.byte_size()
    }

    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        self.0.write(writer)
    }
}

impl<const N: usize, const V: usize> Debug for PaddedByte<N, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let u : u8 = self.as_();
        Debug::fmt(&u, f)
    }
}

impl<const N: usize, const V: usize> PaddedByte<N, V> {
    const MASK: u8 = (1u8 << (8 - N)).wrapping_sub(1);
    const PAD: u8 = if V == 0 { 0 } else { u8::MAX & !Self::MASK };
}

impl<const N: usize, const V: usize> AsPrimitive<u8> for PaddedByte<N, V> {
    fn as_(self) -> u8 {
        self.0 & Self::MASK
    }
}

impl<const N: usize, const V: usize> AsPrimitive<PaddedByte<N, V>> for u8 {
    fn as_(self) -> PaddedByte<N, V> {
        PaddedByte::<N, V>(self & PaddedByte::<N, V>::MASK | PaddedByte::<N, V>::PAD)
    }
}

#[cfg(test)]
mod test {
    use crate::types::padded_byte::PaddedByte;

    #[test]
    fn test_mask() {
        assert_eq!(PaddedByte::<6, 1>::from(0).0, 0b11111100);
        assert_eq!(PaddedByte::<6, 1>::from(3).0, 0b11111111);
    }

}
