
use std::mem::size_of;
use std::str::FromStr;
use chrono::{DateTime, Duration, Utc};
use crate::id::BoxId;
use crate::mp4box::box_full::{FullBox, FullBoxData, FullBoxInfo};
use crate::mp4box::box_root::MP4Box;
use crate::r#type::BoxType;
pub use async_trait::async_trait;
use isolanguage_1::LanguageCode;
use crate::bytes_read::ReadMp4;
use crate::error::MP4Error;
use crate::mp4box::mvhd::base_date;
use crate::bytes_write::{Mp4Writable, WriteMp4};
use crate::error::MalformedBoxError::UnknownVersion;
use crate::mp4box::box_trait::{PartialBox, PartialBoxRead, PartialBoxWrite};

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
                        reader.read::<u32>().await? as i64,
                        reader.read::<u32>().await? as i64,
                        reader.read().await?,
                        Some(reader.read::<u32>().await?).and_then(|it| {
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
                        reader.read::<u64>().await? as i64,
                        reader.read::<u64>().await? as i64,
                        reader.read().await?,
                        Some(reader.read::<u64>().await?).and_then(|it| {
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
        let language = U16LanguageCode::from_u16(reader.read().await?);
        reader.reserve::<u16>().await?;

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
            count += (self.creation_time() as u32).write(writer).await?;
            count += (self.modification_time() as u32).write(writer).await?;
            count += self.timescale.write(writer).await?;
            count += if let Some(duration) = self.duration {
                (duration as u32).write(writer).await?
            } else {
                u32::MAX.write(writer).await?
            }
        } else {
            count += self.creation_time().write(writer).await?;
            count += self.modification_time().write(writer).await?;
            count += self.timescale.write(writer).await?;
            count += if let Some(duration) = self.duration {
                duration.write(writer).await?
            } else {
                u64::MAX.write(writer).await?
            }
        }
        count += self.language.as_u16().write(writer).await?;
        count += writer.reserve::<u16>().await?; // reserved
        Ok(count)
    }
}
