use crate::{full_box, mp4_data};
use crate::types::array::Mp4Array;

mp4_data! {
    #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
    pub struct StcoEntry {
        pub chunk_offset: u32,
    }
}

full_box! {
    box (b"stco", Stco, StcoBox, u32) data {
        entries: Mp4Array<u32, StcoEntry>
    }
}

impl Default for Stco {
    fn default() -> Self {
        Self { entries: Default::default() }
    }
}
