use crate::{full_box, mp4_data};

mp4_data! {
    #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
    pub struct StscEntry {
        pub first_chunk: u32,
        pub samples_per_chunk: u32,
        pub sample_description_index: u32,
    }
}

full_box! {
    box (b"stsc", Stsc, StscBox, u32) data {
        entries: Mp4Array<u32, StscEntry>
    }
}