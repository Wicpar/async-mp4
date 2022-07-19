use std::marker::PhantomData;
use crate::bytes_read::{Mp4Readable, ReadMp4};
use crate::bytes_write::{Mp4Writable, WriteMp4};
use crate::error::MP4Error;
use async_trait::async_trait;
use num_traits::{AsPrimitive};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Mp4Array<I, T>(Vec<T>, PhantomData<I>)
    where
        I: AsPrimitive<usize> + Mp4Readable + Mp4Writable,
        T: Mp4Readable + Mp4Writable,
        usize: AsPrimitive<I>;

impl<I, T> From<Vec<T>> for Mp4Array<I, T> where
    I: AsPrimitive<usize> + Mp4Readable + Mp4Writable,
    T: Mp4Readable + Mp4Writable,
    usize: AsPrimitive<I> {
    fn from(vec: Vec<T>) -> Self {
        Self(vec, Default::default())
    }
}

#[async_trait]
impl<I, T> Mp4Readable for Mp4Array<I, T>
    where
        I: AsPrimitive<usize> + Mp4Readable + Mp4Writable + Send + Sync,
        T: Mp4Readable + Mp4Writable + Send + Sync,
        usize: AsPrimitive<I>
{
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        let i: I = reader.read().await?;
        let mut vec = vec![];
        for _ in 0..AsPrimitive::<usize>::as_(i) {
            vec.push(reader.read().await?);
        }
        Ok(Self(vec, Default::default()))
    }
}

#[async_trait]
impl<I, T> Mp4Writable for Mp4Array<I, T>
    where
        I: AsPrimitive<usize> + Mp4Readable + Mp4Writable + Send + Sync,
        T: Mp4Readable + Mp4Writable + Send + Sync,
        usize: AsPrimitive<I>
{
    fn byte_size(&self) -> usize {
        AsPrimitive::<I>::as_(self.0.len()).byte_size() + self.0.iter().map(Mp4Writable::byte_size).sum::<usize>()
    }

    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        count += AsPrimitive::<I>::as_(self.0.len()).write(writer).await?;
        for elem in &self.0 {
            count += elem.write(writer).await?;
        }
        Ok(count)
    }
}
