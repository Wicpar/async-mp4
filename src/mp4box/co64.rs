use crate::{full_box, mp4_data};

mp4_data! {
    #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
    pub struct StcoEntry {
        pub chunk_offset: u64,
    }
}

full_box! {
    box (b"co64", Co64, Co64Box, u32) data {
        entries: Mp4Array<u32, StcoEntry>
    }
}
