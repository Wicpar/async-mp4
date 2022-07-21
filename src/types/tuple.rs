use async_trait::async_trait;
use crate::bytes_read::{Mp4Readable, ReadMp4};
use crate::bytes_write::{Mp4Writable, WriteMp4};
use crate::error::MP4Error;

macro_rules! impl_tuple {
    ($($t:ident),+) => {

        #[async_trait]
        impl<$($t: Mp4Readable + Send + Sync),+> Mp4Readable for ($($t,)+) {
            async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
                Ok((
                    $(reader.read::<$t>().await?,)+
                ))
            }
        }

        #[async_trait]
        #[allow(non_snake_case)]
        impl<$($t: Mp4Writable + Send + Sync),+> Mp4Writable for ($($t,)+) {
            fn byte_size(&self) -> usize {
                let ($($t,)+) = self;
                $($t.byte_size() + )+ 0
            }

            async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
                let mut count = 0;
                let ($($t,)+) = self;
                $(count += $t.write(writer).await?;)+
                Ok(count)
            }
        }
    }
}

impl_tuple!(A);
impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);
impl_tuple!(A, B, C, D, E);
impl_tuple!(A, B, C, D, E, F);
impl_tuple!(A, B, C, D, E, F, G);
