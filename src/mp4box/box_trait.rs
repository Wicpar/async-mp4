use async_trait::async_trait;
use crate::bytes_read::ReadMp4;
use crate::bytes_write::WriteMp4;
use crate::error::MP4Error;
use crate::header::BoxHeader;
use crate::r#type::BoxType;

pub trait IBox {
    fn byte_size(&self) -> usize;
    const ID: BoxType;
}

impl<T: IBox> IBox for Vec<T> {
    fn byte_size(&self) -> usize {
        self.iter().map(IBox::byte_size).sum()
    }

    const ID: BoxType = T::ID;
}

impl<T: IBox> IBox for Option<T> {
    fn byte_size(&self) -> usize {
        self.iter().map(IBox::byte_size).sum()
    }

    const ID: BoxType = T::ID;
}

#[async_trait]
pub trait BoxRead<R: ReadMp4>: IBox + Sized {
    async fn read(header: BoxHeader, reader: &mut R) -> Result<Self, MP4Error>;
}

#[async_trait]
pub trait BoxWrite<W: WriteMp4>: IBox {
    async fn write(&self, writer: &mut W) -> Result<usize, MP4Error>;
}

#[async_trait]
impl<T: BoxWrite<W> + Sync, W: WriteMp4> BoxWrite<W> for Vec<T> {
    async fn write(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        for item in self {
            count += item.write(writer).await?;
        }
        Ok(count)
    }
}

#[async_trait]
impl<T: BoxWrite<W> + Sync, W: WriteMp4> BoxWrite<W> for Option<T> {
    async fn write(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        if let Some(item) = self {
            count += item.write(writer).await?;
        }
        Ok(count)
    }
}

pub trait PartialBox {
    type ParentData;
    type ThisData;
    fn byte_size(&self) -> usize;
    const ID: BoxType;
}

#[async_trait]
pub trait PartialBoxRead<R: ReadMp4>: PartialBox + Sized {
    async fn read_data(parent_data: Self::ParentData, reader: &mut R) -> Result<Self, MP4Error>;
    async fn read_child(&mut self, _header: BoxHeader, _reader: &mut R) -> Result<(), MP4Error> {
        Ok(())
    }
}

#[async_trait]
pub trait PartialBoxWrite<W: WriteMp4>: PartialBox {
    async fn write_data(&self, _writer: &mut W) -> Result<usize, MP4Error> {Ok(0)}
    async fn write_children(&self, _writer: &mut W) -> Result<usize, MP4Error> {Ok(0)}
}