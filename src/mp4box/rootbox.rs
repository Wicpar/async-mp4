use async_trait::async_trait;
use std::io::SeekFrom;
use std::ops::Deref;
use futures::{AsyncRead, AsyncSeek, AsyncSeekExt, AsyncWrite};
use crate::error::MalformedBoxError::{ReadingWrongBox, UnknownSizeForUnknownBox};
use crate::error::MP4Error;
use crate::header::BoxHeader;
use crate::mp4box::{BoxRead, BoxWrite, IBox, PartialBox, PartialBoxRead, PartialBoxWrite};
use crate::r#type::BoxType;
use crate::size::BoxSize;
use crate::size::BoxSize::Known;

#[derive(Debug, Clone)]
pub struct MP4Box<P>
    where
        P: PartialBox<ParentData=()>
{
    pub inner: P,
}

impl<P> From<P> for MP4Box<P> where
    P: PartialBox<ParentData=()>
{
    fn from(inner: P) -> Self {
        Self{inner}
    }
}

impl<P> MP4Box<P>
    where
        P: PartialBox<ParentData=()>
{
    fn header(&self) -> BoxHeader {
        BoxHeader::from_id_and_inner_size(P::id(), self.inner.byte_size())
    }
}

#[async_trait]
impl<P, W> BoxWrite<W> for MP4Box<P>
    where
        P: PartialBox<ParentData=()> + PartialBoxWrite<W> + Send + Sync,
        W: AsyncWrite + Unpin + Send + Sync
{
    async fn write(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        count += self.header().write(writer).await?;
        count += self.inner.write_data(writer).await?;
        count += self.inner.write_children(writer).await?;
        debug_assert!(count == self.byte_size(), "Byte Size is not equal to written size");
        Ok(count)
    }
}

impl<P> IBox for MP4Box<P>
    where
        P: PartialBox<ParentData=()>
{
    fn byte_size(&self) -> usize {
        self.header().byte_size() + self.inner.byte_size()
    }

    fn id() -> BoxType {
        P::id()
    }
}

#[async_trait]
impl<P, R> BoxRead<R> for MP4Box<P>
    where
        P: PartialBox<ParentData=()> + PartialBoxRead<R> + Send + Sync,
        R: AsyncRead + AsyncSeek + Unpin + Send + Sync
{
    async fn read(header: BoxHeader, reader: &mut R) -> Result<Self, MP4Error> {
        let actual = header.id;
        let  target = Self::id();
        if actual != target {
            return Err(ReadingWrongBox {actual, target}.into())
        }
        let start = reader.seek(SeekFrom::Current(0)).await? - header.byte_size() as u64;
        let size = header.size;
        let mut inner = P::read_data((), reader).await?;
        while !size.ended(start, reader).await? {
            let header = BoxHeader::read(reader).await?;
            let pos = reader.seek(SeekFrom::Current(0)).await?;
            let size = header.size_minus_self();
            inner.read_child(header, reader).await?;
            if let Known(size) = size { // we do the check here because it's far safer
                reader.seek(SeekFrom::Start(pos + size as u64)).await?;
            } else {
                return Err(UnknownSizeForUnknownBox.into());
            }
        }
        Ok(Self { inner })
    }
}

impl<P> Deref for MP4Box<P>
    where
        P: PartialBox<ParentData=()>
{
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}