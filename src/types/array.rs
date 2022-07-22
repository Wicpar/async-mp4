use std::cmp::max;
use std::marker::PhantomData;
use crate::bytes_read::{Mp4Readable, Mp4VersionedReadable, ReadMp4};
use crate::bytes_write::{FlagTrait, Mp4VersionedWritable, Mp4Writable, WriteMp4};
use crate::error::MP4Error;
use async_trait::async_trait;
use num_traits::{AsPrimitive};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Mp4Array<I, T>(pub Vec<T>, pub PhantomData<I>)
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

impl<I, T> Mp4Writable for Mp4Array<I, T>
    where
        I: AsPrimitive<usize> + Mp4Readable + Mp4Writable + Send + Sync,
        T: Mp4Readable + Mp4Writable + Send + Sync,
        usize: AsPrimitive<I>
{
    fn byte_size(&self) -> usize {
        AsPrimitive::<I>::as_(self.0.len()).byte_size() + self.0.iter().map(Mp4Writable::byte_size).sum::<usize>()
    }

    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        count += AsPrimitive::<I>::as_(self.0.len()).write(writer)?;
        for elem in &self.0 {
            count += elem.write(writer)?;
        }
        Ok(count)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Mp4OffsetArray<I, O, T>
    where
        I: AsPrimitive<usize> + Mp4Readable + Mp4Writable,
        usize: AsPrimitive<I>
{
    pub data: Vec<T>,
    pub offset: O,
    pub _p: PhantomData<I>,
}

impl<I, O, T> Mp4OffsetArray<I, O, T> where
    I: AsPrimitive<usize> + Mp4Readable + Mp4Writable,
    usize: AsPrimitive<I> {

    pub fn new(data: Vec<T>, offset: O) -> Self {
        Self { data, offset, _p: Default::default() }
    }

}

#[async_trait]
impl<I, O, T> Mp4Readable for Mp4OffsetArray<I, O, T> where
    I: AsPrimitive<usize> + Mp4Readable + Mp4Writable + Send + Sync,
    T: Mp4Readable + Mp4Writable + Send + Sync,
    O: Mp4Readable + Mp4Writable + Send + Sync,
    usize: AsPrimitive<I>
{
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        let i: I = reader.read().await?;
        let offset: O = reader.read().await?;
        let mut data = vec![];
        for _ in 0..AsPrimitive::<usize>::as_(i) {
            data.push(reader.read().await?);
        }
        Ok(Self { data, offset, _p: Default::default() })
    }
}

impl<I, O, T> Mp4Writable for Mp4OffsetArray<I, O, T> where
    I: AsPrimitive<usize> + Mp4Readable + Mp4Writable + Send + Sync,
    T: Mp4Readable + Mp4Writable + Send + Sync,
    O: Mp4Readable + Mp4Writable + Send + Sync,
    usize: AsPrimitive<I>
{
    fn byte_size(&self) -> usize {
        AsPrimitive::<I>::as_(self.data.len()).byte_size() + self.data.iter().map(Mp4Writable::byte_size).sum::<usize>() + self.offset.byte_size()
    }

    fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        count += AsPrimitive::<I>::as_(self.data.len()).write(writer)?;
        count += self.offset.write(writer)?;
        for elem in &self.data {
            count += elem.write(writer)?;
        }
        Ok(count)
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Mp4VersionedOffsetArray<I, O, T>
    where
        I: AsPrimitive<usize> + Mp4Readable + Mp4Writable,
        usize: AsPrimitive<I>
{
    pub data: Vec<T>,
    pub offset: O,
    pub _p: PhantomData<I>,
}

impl<I, O, T> Mp4VersionedOffsetArray<I, O, T> where
    I: AsPrimitive<usize> + Mp4Readable + Mp4Writable,
    usize: AsPrimitive<I> {

    pub fn new(data: Vec<T>, offset: O) -> Self {
        Self { data, offset, _p: Default::default() }
    }

}

#[async_trait]
impl<I, O, T, F> Mp4VersionedReadable<F> for Mp4VersionedOffsetArray<I, O, T> where
    I: AsPrimitive<usize> + Mp4Readable + Mp4Writable + Send + Sync,
    T: Mp4VersionedReadable<F> + Mp4VersionedWritable<F> + Send + Sync,
    O: Mp4VersionedReadable<F> + Mp4VersionedWritable<F> + Send + Sync,
    F: FlagTrait,
    usize: AsPrimitive<I>
{
    async fn versioned_read<R: ReadMp4>(version: u8, flags: F, reader: &mut R) -> Result<Self, MP4Error> {
        let i: I = reader.read().await?;
        let offset: O = reader.versioned_read(version, flags).await?;
        let mut data = vec![];
        for _ in 0..AsPrimitive::<usize>::as_(i) {
            data.push(reader.versioned_read(version, flags).await?);
        }
        Ok(Self { data, offset, _p: Default::default() })
    }
}

impl<I, O, T, F> Mp4VersionedWritable<F> for Mp4VersionedOffsetArray<I, O, T> where
    I: AsPrimitive<usize> + Mp4Readable + Mp4Writable + Send + Sync,
    T: Mp4VersionedReadable<F> + Mp4VersionedWritable<F> + Send + Sync,
    O: Mp4VersionedReadable<F> + Mp4VersionedWritable<F> + Send + Sync,
    F: FlagTrait,
    usize: AsPrimitive<I>
{
    fn required_version(&self) -> u8 {
        max(self.offset.required_version(), self.data.iter().map(Mp4VersionedWritable::required_version).max().unwrap_or_default())
    }

    fn required_flags(&self) -> F {
        self.offset.required_flags() | self.data.iter().map(Mp4VersionedWritable::required_flags).reduce(F::bitor).unwrap_or_default()
    }

    fn versioned_byte_size(&self, version: u8, flags: F) -> usize {
        AsPrimitive::<I>::as_(self.data.len()).byte_size() +
            self.offset.versioned_byte_size(version, flags) +
            self.data.iter().map(|it|it.versioned_byte_size(version, flags)).sum::<usize>()
    }

    fn versioned_write<W: WriteMp4>(&self, version: u8, flags: F, writer: &mut W) -> Result<usize, MP4Error> {
        let mut count = 0;
        count += AsPrimitive::<I>::as_(self.data.len()).write(writer)?;
        count += self.offset.versioned_write(version, flags, writer)?;
        for elem in &self.data {
            count += elem.versioned_write(version, flags, writer)?;
        }
        Ok(count)
    }
}
