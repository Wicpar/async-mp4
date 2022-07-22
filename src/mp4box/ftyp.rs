use std::io::SeekFrom;
use futures::{AsyncSeekExt};
use crate::bytes_read::ReadMp4;
use crate::bytes_write::{Mp4Writable, WriteMp4};
use crate::error::MP4Error;
use crate::header::BoxHeader;
use crate::id::BoxId;
use crate::mp4box::box_trait::{BoxRead, BoxWrite, IBox};
use crate::r#type::BoxType;

pub type FtypBox = Ftyp;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Ftyp {
    pub major_brand: [u8; 4],
    pub minor_version: u32,
    pub compatible_brands: Vec<[u8; 4]>
}

impl Ftyp {
    fn inner_byte_size(&self) -> usize {
        self.major_brand.byte_size() + self.minor_version.byte_size() + self.compatible_brands.iter().map(Mp4Writable::byte_size).sum::<usize>()
    }

    fn header(&self) -> BoxHeader {
        BoxHeader::from_id_and_inner_size(Self::ID, self.inner_byte_size())
    }
}

impl IBox for Ftyp {
    fn byte_size(&self) -> usize {
        self.header().byte_size() + self.inner_byte_size()
    }

    const ID: BoxType = BoxType::Id(BoxId(*b"ftyp"));
}

#[async_trait::async_trait]
impl BoxRead for Ftyp {
    async fn read<R: ReadMp4>(header: BoxHeader, reader: &mut R) -> Result<Self, MP4Error> {
        let start = reader.seek(SeekFrom::Current(0)).await?;
        let size = header.size_minus_self();
        let major_brand = reader.read().await?;
        let minor_version = reader.read().await?;
        let mut compatible_brands = vec![];
        while !size.ended(start, reader).await? {
            compatible_brands.push(reader.read().await?);
        }
        Ok(Self {
            major_brand,
            minor_version,
            compatible_brands
        })
    }
}

impl BoxWrite for Ftyp {

    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        count += self.header().write(writer)?;
        count += self.major_brand.write(writer)?;
        count += self.minor_version.write(writer)?;
        count += self.compatible_brands.write(writer)?;
        Ok(count)
    }
}
