use fixed::types::I8F8;
use crate::full_box;

full_box! {
    box (b"smhd", Smhd, SmhdBox, u32)
    data {
        balance: I8F8,
        _r1: u16
    }
}

impl Default for Smhd {
    fn default() -> Self {
        Self {
            balance: Default::default(),
            _r1: 0
        }
    }
}
