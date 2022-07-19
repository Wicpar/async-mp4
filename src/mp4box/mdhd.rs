use crate::types::date::Mp4DateTime;
use crate::full_box;
use crate::types::duration::Mp4Duration;
use crate::types::language::Mp4LanguageCode;

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
