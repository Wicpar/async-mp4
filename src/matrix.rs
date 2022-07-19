use fixed::types::{I16F16, I2F30};
use fixed_macro::fixed;
use crate::mp4_data;

mp4_data! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct MP4Matrix {
        pub a: I16F16,
        pub b: I16F16,
        pub u: I2F30,
        pub c: I16F16,
        pub d: I16F16,
        pub v: I2F30,
        pub x: I16F16,
        pub y: I16F16,
        pub w: I2F30,
    }
}

impl MP4Matrix {
    pub fn byte_size() -> usize {
        4 * 9
    }

    pub fn unity() -> MP4Matrix {
        Self {
            a: fixed!(1: I16F16),
            b: fixed!(0: I16F16),
            u: fixed!(0: I2F30),
            c: fixed!(0: I16F16),
            d: fixed!(1: I16F16),
            v: fixed!(0: I2F30),
            x: fixed!(0: I16F16),
            y: fixed!(0: I16F16),
            w: fixed!(1: I2F30),
        }
    }
}

impl Default for MP4Matrix {
    fn default() -> Self {
        Self::unity()
    }
}
