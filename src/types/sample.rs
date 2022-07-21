use std::ops::Deref;
use fixed::types::I16F16;
use crate::mp4_data;

mp4_data! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SampleEntry {
        pub _r1: [u8; 6],
        pub data_reference_index: u16
    }
}

mp4_data! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct VisualSampleEntry {
        pub sample_entry: SampleEntry,
        pub _r1: [u32; 4],
        pub width: u16,
        pub height: u16,
        pub horizresolution: I16F16,
        pub vertresolution: I16F16,
        pub _r2: u32,
        pub framecount: u16,
        pub compressorname: [u8; 16],
        pub depth: u16,
        pub _r3: u16,
    }
}

impl Deref for VisualSampleEntry {
    type Target = SampleEntry;

    fn deref(&self) -> &Self::Target {
        &self.sample_entry
    }
}

mp4_data! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AudioSampleEntry {
        pub sample_entry: SampleEntry,
        pub _r1: [u32; 2],
        pub channel_count: u16,
        pub sample_size: u16,
        pub _r2: [u16; 2],
        pub sample_rate: I16F16,
    }
}

impl Deref for AudioSampleEntry {
    type Target = SampleEntry;

    fn deref(&self) -> &Self::Target {
        &self.sample_entry
    }
}
