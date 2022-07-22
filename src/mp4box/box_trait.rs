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
pub trait BoxRead: IBox + Sized {
    async fn read<R: ReadMp4>(header: BoxHeader, reader: &mut R) -> Result<Self, MP4Error>;
}

pub trait BoxWrite: IBox {
    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error>;
}

impl<T: BoxWrite> BoxWrite for Vec<T> {
    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        for item in self {
            count += item.write(writer)?;
        }
        Ok(count)
    }
}

impl<T: BoxWrite> BoxWrite for Option<T> {
    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        if let Some(item) = self {
            count += item.write(writer)?;
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
pub trait PartialBoxRead: PartialBox + Sized {
    async fn read_data<R: ReadMp4>(parent_data: Self::ParentData, reader: &mut R) -> Result<Self, MP4Error>;
    async fn read_child<R: ReadMp4>(&mut self, _header: BoxHeader, _reader: &mut R) -> Result<(), MP4Error> {
        Ok(())
    }
}

pub trait PartialBoxWrite: PartialBox {
    fn write_data<W: WriteMp4>(&self, _writer: &mut W) -> Result<usize, MP4Error> {Ok(0)}
    fn write_children<W: WriteMp4>(&self, _writer: &mut W) -> Result<usize, MP4Error> {Ok(0)}
}
