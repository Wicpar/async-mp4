use fixed_macro::fixed;
use crate::full_box;
use crate::types::date::Mp4DateTime;
use crate::types::duration::Mp4Duration;
use fixed::types::I8F8;
use fixed::types::I16F16;
use crate::matrix::MP4Matrix;

full_box! {
    box (b"mvhd", Mvhd, MvhdBox, u32)
    data {
        creation_time: Mp4DateTime,
        modification_time: Mp4DateTime,
        timescale: u32,
        duration: Mp4Duration,
        rate: I16F16,
        volume: I8F8,
        _r1: u16,
        _r2: [u32; 2],
        matrix: MP4Matrix,
        _r3: [u32; 6],
        next_track_id: u32
    }
}

impl Default for Mvhd {
    fn default() -> Self {
        Self {
            creation_time: Default::default(),
            modification_time: Default::default(),
            timescale: 1000,
            duration: Default::default(),
            rate: fixed!(1: I16F16),
            volume: fixed!(1: I8F8),
            _r1: Default::default(),
            _r2: Default::default(),
            matrix: Default::default(),
            _r3: Default::default(),
            next_track_id: 1
        }
    }
}
