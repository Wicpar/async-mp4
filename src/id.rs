use std::fmt::{Debug, Display, Formatter};
use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use crate::error::MP4Error;

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

    pub async fn read<R: AsyncRead + Unpin>(reader: &mut R) -> Result<BoxId, MP4Error> {
        let mut data = [0u8; 4];
        reader.read_exact(&mut data).await?;
        Ok(Self(data))
    }

    pub async fn write<W: AsyncWrite + Unpin>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        writer.write_all(&self.0).await?;
        Ok(4)
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
