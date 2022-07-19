use std::fmt::{Debug, Formatter};
use std::str::FromStr;
pub use async_trait::async_trait;
use isolanguage_1::LanguageCode;
use crate::bytes_read::{Mp4Readable, ReadMp4};
use crate::error::MP4Error;
use crate::mp4box::mvhd::{ Mp4DateTime, Mp4Duration};
use crate::bytes_write::{Mp4Writable, WriteMp4};
use crate::full_box;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Mp4LanguageCode(u16);

impl Debug for Mp4LanguageCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.to_language_code(), f)
    }
}
impl Default for Mp4LanguageCode {
    fn default() -> Self {
        Self::from_language_code(None)
    }
}

impl From<LanguageCode> for Mp4LanguageCode {
    fn from(code: LanguageCode) -> Self {
        Self::from_language_code(Some(code))
    }
}

impl From<Option<LanguageCode>> for Mp4LanguageCode {
    fn from(code: Option<LanguageCode>) -> Self {
        Self::from_language_code(code)
    }
}

impl From<Mp4LanguageCode> for Option<LanguageCode> {
    fn from(code: Mp4LanguageCode) -> Self {
        code.to_language_code()
    }
}


impl Mp4LanguageCode {
    fn to_language_code(&self) -> Option<LanguageCode> {
        const MASK: u8 = 0b11111;
        let value = self.0;
        let data = [
            ((value >> 10) as u8 & MASK) + 0x60,
            ((value >> 05) as u8 & MASK) + 0x60,
            ((value >> 00) as u8 & MASK) + 0x60
        ];
        LanguageCode::from_str(std::str::from_utf8(&data).ok()?).ok()
    }

    fn from_language_code(code: Option<LanguageCode>) -> Self {
        const UND: u16 =
            ((b'u' as u16 - 0x60) << 10) |
            ((b'n' as u16 - 0x60) << 05) |
            ((b'd' as u16 - 0x60) << 00);

        Self(match code {
            None => UND,
            Some(code) => {
                if let [a, b, c] = code.code_t().as_bytes() {
                    ((*a as u16 - 0x60) << 10) | ((*b as u16 - 0x60) << 05) | ((*c as u16 - 0x60) << 00)
                } else {
                    UND
                }
            }
        })
    }
}

#[async_trait]
impl Mp4Readable for Mp4LanguageCode {
    async fn read<R: ReadMp4>(reader: &mut R) -> Result<Self, MP4Error> {
        Ok(Self(reader.read().await?))
    }
}

#[async_trait]
impl Mp4Writable for Mp4LanguageCode {
    fn byte_size(&self) -> usize {
        self.0.byte_size()
    }

    async fn write<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
        self.0.write(writer).await
    }
}

full_box! {
    box (b"mdhd", Mdhd, MdhdBox, u32)
    data {
        creation_time: Mp4DateTime,
        modification_time: Mp4DateTime,
        timescale: u32,
        duration: Mp4Duration,
        language: Mp4LanguageCode,
        _r1: u16
    }
}

impl Default for Mdhd {
    fn default() -> Self {
        Self {
            creation_time: Default::default(),
            modification_time: Default::default(),
            timescale: 1000,
            duration: Default::default(),
            language: Default::default(),
            _r1: Default::default()
        }
    }
}
