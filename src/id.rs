use std::fmt::{Debug, Display, Formatter};
use crate::bytes_read::{Mp4Readable, ReadMp4};
use crate::error::MP4Error;
use async_trait::async_trait;
use crate::bytes_write::{Mp4Writable, WriteMp4};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BoxId(pub [u8; 4]);

impl Debug for BoxId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for BoxId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match std::str::from_utf8(&self.0) {
            Ok(str) => Display::fmt(str, f),
            Err(_) => Display::fmt(&self.0.escape_ascii(), f)
        }
    }
}

impl BoxId {
    pub const fn size() -> usize {
        4
    }
}

#[async_trait]
impl Mp4Readable for BoxId {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        Ok(Self(reader.read().await?))
    }
}

impl Mp4Writable for BoxId {
    fn byte_size(&self) -> usize {
        self.0.byte_size()
    }

    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        self.0.write(writer)
    }
}


impl PartialEq<&[u8;4]> for BoxId {
    fn eq(&self, other: &&[u8;4]) -> bool {
        &self.0 == *other
    }
}

impl From<[u8;4]> for BoxId {
    fn from(id: [u8; 4]) -> Self {
        Self(id)
    }
}

impl From<&[u8;4]> for BoxId {
    fn from(id: &[u8; 4]) -> Self {
        Self(*id)
    }
}
