use crate::error::MP4Error;
use async_trait::async_trait;

pub trait Mp4Reservable {
    const BYTE_SIZE: usize;
}


impl Mp4Reservable for u8 { const BYTE_SIZE: usize = 1;}
impl Mp4Reservable for u16 { const BYTE_SIZE: usize = 2;}
impl Mp4Reservable for u32 { const BYTE_SIZE: usize = 4;}
impl Mp4Reservable for u64 { const BYTE_SIZE: usize = 8;}
impl Mp4Reservable for i8 { const BYTE_SIZE: usize = 1;}
impl Mp4Reservable for i16 { const BYTE_SIZE: usize = 2;}
impl Mp4Reservable for i32 { const BYTE_SIZE: usize = 4;}
impl Mp4Reservable for i64 { const BYTE_SIZE: usize = 8;}

impl<T: Mp4Reservable, const N: usize> Mp4Reservable for [T; N] { const BYTE_SIZE: usize = T::BYTE_SIZE * N;}

#[async_trait]
pub trait Mp4Reserve {
    async fn reserve<T: Mp4Reservable>(&mut self) -> Result<usize, MP4Error>;
}

