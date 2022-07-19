use async_trait::async_trait;
use crate::bytes_read::{Mp4Readable, ReadMp4};
use crate::bytes_write::{Mp4Writable, WriteMp4};
use crate::error::MP4Error;

#[async_trait]
impl Mp4Readable for String {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        let mut name = vec![];
        loop {
            match reader.read().await? {
                0 => break,
                it => name.push(it)
            }
        }
        Ok(String::from_utf8(name)?)
    }
}

#[async_trait]
impl Mp4Writable for String {
    fn byte_size(&self) -> usize {
        self.as_bytes().len() + 1
    }

    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        count += self.as_bytes().write(writer).await?;
        count += 0u8.write(writer).await?;
        Ok(count)
    }
}
