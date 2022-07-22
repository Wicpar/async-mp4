use crate::{full_box, mp4_data};
use crate::types::array::Mp4Array;

mp4_data! {
    #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
    pub struct SttsEntry {
        pub sample_count: u32,
        pub sample_delta: u32
    }
}

full_box! {
    box (b"stts", Stts, SttsBox, u32)
    data {
        samples: Mp4Array<u32, SttsEntry>
    }
}

impl Default for Stts {
    fn default() -> Self {
        Self {
            samples: Default::default()
        }
    }
}
