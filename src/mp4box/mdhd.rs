
use std::mem::size_of;
use std::str::FromStr;
use chrono::{DateTime, Duration, Utc};
use crate::id::BoxId;
use crate::mp4box::full_box::{FullBox, FullBoxData, FullBoxInfo};
use crate::mp4box::{PartialBox, PartialBoxRead, PartialBoxWrite};
use crate::mp4box::rootbox::MP4Box;
use crate::r#type::BoxType;
pub use async_trait::async_trait;
use isolanguage_1::LanguageCode;
use crate::bytes_read::ReadMp4;
use crate::error::MP4Error;
use crate::mp4box::mvhd::base_date;
use crate::bytes_write::WriteMp4;
use crate::error::MalformedBoxError::UnknownVersion;

pub type MdhdBox = MP4Box<FullBox<Mdhd>>;

pub trait U16LanguageCode {
    fn from_u16(value: u16) -> Self;
    fn as_u16(&self) -> u16;
}

impl U16LanguageCode for Option<LanguageCode> {
    fn from_u16(value: u16) -> Self {
        const MASK: u8 = 0b11111;
        let data = [
            ((value >> 10) as u8 & MASK) + 0x60,
            ((value >> 05) as u8 & MASK) + 0x60,
            ((value >> 00) as u8 & MASK) + 0x60
        ];
        LanguageCode::from_str(std::str::from_utf8(&data).ok()?).ok()
    }

    fn as_u16(&self) -> u16 {
        const UND: u16 =
            ((b'u' as u16 - 0x60) << 10) |
            ((b'n' as u16 - 0x60) << 05) |
            ((b'd' as u16 - 0x60) << 00);

        match self {
            None => UND,
            Some(code) => {
                if let [a, b, c] = code.code_t().as_bytes() {
                    ((*a as u16 - 0x60) << 10) | ((*b as u16 - 0x60) << 05) | ((*c as u16 - 0x60) << 00)
                } else {
                    UND
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mdhd {
    creation_time: DateTime<Utc>,
    modification_time: DateTime<Utc>,
    timescale: u32,
    duration: Option<u64>,
    language: Option<LanguageCode>
}

impl Mdhd {
    fn creation_time(&self) -> u64 {
        self.creation_time.signed_duration_since(base_date()).num_seconds() as u64
    }
    fn modification_time(&self) -> u64 {
        self.modification_time.signed_duration_since(base_date()).num_seconds() as u64
    }
}

impl Default for Mdhd {
    fn default() -> Self {
        Self {
            creation_time: Utc::now(),
            modification_time: Utc::now(),
            timescale: 1000,
            duration: None,
            language: None,
        }
    }
}

impl FullBoxInfo for Mdhd {
    fn version(&self) -> u8 {
        let large = self.creation_time() > u32::MAX as u64 ||
            self.modification_time() > u32::MAX as u64 ||
            self.duration.map(|it|it > u32::MAX as u64).unwrap_or(false);
        if large { 1 } else { 0 }
    }
}

impl PartialBox for Mdhd {
    type ParentData = FullBoxData;

    fn byte_size(&self) -> usize {
        let version = self.version();
        let mut base = if version == 1 {
            3 * size_of::<u64>() +
            1 * size_of::<u32>()
        } else {
            4 * size_of::<u32>()
        };
        base += size_of::<u16>(); // language
        base += size_of::<u16>(); // reserved
        base
    }

    const ID: BoxType = BoxType::Id(BoxId(*b"mdhd"));
}

#[async_trait]
impl<R: ReadMp4> PartialBoxRead<R> for Mdhd {
    async fn read_data(data: Self::ParentData, reader: &mut R) -> Result<Self, MP4Error> {
        let base_time = base_date();
        let (creation_time, modification_time, timescale, duration) =
            match data.version {
                0 => {
                    (
                        reader.read_u32().await? as i64,
                        reader.read_u32().await? as i64,
                        reader.read_u32().await?,
                        Some(reader.read_u32().await?).and_then(|it| {
                            if it == u32::MAX {
                                None
                            } else {
                                Some(it as u64)
                            }
                        }),
                    )
                }
                1 => {
                    (
                        reader.read_u64().await? as i64,
                        reader.read_u64().await? as i64,
                        reader.read_u32().await?,
                        Some(reader.read_u64().await?).and_then(|it| {
                            if it == u64::MAX {
                                None
                            } else {
                                Some(it as u64)
                            }
                        }),
                    )
                }
                _ => return Err(UnknownVersion(Self::ID, data.version).into())
            };
        let language = U16LanguageCode::from_u16(reader.read_u16().await?);
        reader.reserved(size_of::<u16>()).await?;

        let creation_time = base_time.clone() + Duration::seconds(creation_time);
        let modification_time = base_time.clone() + Duration::seconds(modification_time);

        Ok(Self {
            creation_time,
            modification_time,
            timescale,
            duration,
            language
        })
    }
}

#[async_trait]
impl<W: WriteMp4> PartialBoxWrite<W> for Mdhd {

    async fn write_data(&self, writer: &mut W) -> Result<usize, MP4Error> {
        let version = self.version();
        let mut count = 0;
        if version == 0 {
            count += writer.write_u32(self.creation_time() as _).await?;
            count += writer.write_u32(self.modification_time() as _).await?;
            count += writer.write_u32(self.timescale).await?;
            count += if let Some(duration) = self.duration {
                writer.write_u32(duration as _).await?
            } else {
                writer.write_u32(u32::MAX).await?
            }
        } else {
            count += writer.write_u64(self.creation_time()).await?;
            count += writer.write_u64(self.modification_time()).await?;
            count += writer.write_u32(self.timescale).await?;
            count += if let Some(duration) = self.duration {
                writer.write_u64(duration).await?
            } else {
                writer.write_u64(u64::MAX).await?
            }
        }
        count += writer.write_u16(self.language.as_u16()).await?;
        count += writer.reserved(size_of::<u16>()).await?; // reserved
        Ok(count)
    }
}
