use std::hash::{Hash, Hasher};
pub use async_trait::async_trait;
use bitregions::bitregions;
use fixed::types::I8F8;
use fixed_macro::fixed;
use crate::error::MP4Error;
use crate::matrix::MP4Matrix;
use crate::full_box;
use crate::mp4box::mvhd::Mp4DateTime;
use crate::mp4box::mvhd::Mp4Duration;

bitregions! {
    pub TrakFlags u32 {
        ENABLED:                0b0001,
        IN_MOVIE:               0b0010,
        IN_PREVIEW:             0b0100,
        SIZE_IS_ASPECT_RATIO:   0b1000,
    }
}
impl Eq for TrakFlags {}
impl Hash for TrakFlags {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}


full_box! {
    box (b"tkhd", Tkhd, TkhdBox, @save flags: TrakFlags)
    data {
        creation_time: Mp4DateTime,
        modification_time: Mp4DateTime,
        track_id: u32,
        _r1: u32,
        duration: Mp4Duration,
        _r2: [u32; 2],
        layer: i16,
        alternate_group: i16,
        volume: I8F8,
        _r3: u16,
        matrix: MP4Matrix,
        width: u32,
        height: u32
    }
}

impl Default for Tkhd {
    fn default() -> Self {
        Self {
            flags: TrakFlags::default() | TrakFlags::ENABLED,
            creation_time:  Default::default(),
            modification_time:  Default::default(),
            track_id: 1,
            _r1:  Default::default(),
            duration: Default::default(),
            _r2:  Default::default(),
            layer: 0,
            alternate_group: 0,
            volume: fixed!(1: I8F8),
            _r3:  Default::default(),
            matrix: Default::default(),
            width: 0,
            height: 0
        }
    }
}
