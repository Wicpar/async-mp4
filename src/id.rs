use crate::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, MP4Error};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BoxId([u8; 4]);

impl BoxId {
    pub async fn read<R: AsyncRead>(mut reader: R) -> Result<BoxId, MP4Error> {
        let mut data = [0u8; 4];
        reader.read_exact(&mut data).await?;
        Ok(Self(data))
    }

    pub fn write<W: AsyncWrite>(&self, mut writer: W) -> Result<usize, MP4Error> {
        writer.write_all(&self.0).await?;
        Ok(4)
    }
}
