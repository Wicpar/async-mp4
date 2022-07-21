use std::hash::{Hash, Hasher};
use bitregions::bitregions;
use crate::{flag_option, full_box, mp4_versioned_data};
use crate::types::versioned_signed_int::VersionedSignedU32;
use crate::mp4box::trex::SampleFlags;
use crate::types::array::{Mp4VersionedOffsetArray};

bitregions! {
    pub TrunFlags u32 {
        HAS_DATA_OFFSET:                0b0000000000000000__0000_0000_0000_0001,
        HAS_FIRST_SAMPLE_FLAGS:         0b0000000000000000__0000_0000_0000_0100,
        HAS_SAMPLE_DURATION:            0b0000000000000000__0000_0001_0000_0000,
        HAS_SAMPLE_SIZE:                0b0000000000000000__0000_0010_0000_0000,
        HAS_SAMPLE_FLAGS:               0b0000000000000000__0000_0100_0000_0000,
        HAS_SAMPLE_COMPOSITION:         0b0000000000000000__0000_1000_0000_0000,
    }
}

impl Eq for TrunFlags {}
impl Hash for TrunFlags {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

flag_option! {
    #[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TrunDataOffset(pub u64, TrunFlags, HAS_DATA_OFFSET);
}

flag_option! {
    #[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TrunFirstSampleFlags(pub SampleFlags, TrunFlags, HAS_FIRST_SAMPLE_FLAGS);
}

flag_option! {
    #[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TrunSampleDuration(pub u32, TrunFlags, HAS_SAMPLE_DURATION);
}

flag_option! {
    #[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TrunSampleSize(pub u32, TrunFlags, HAS_SAMPLE_SIZE);
}

flag_option! {
    #[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TrunSampleFlags(pub SampleFlags, TrunFlags, HAS_SAMPLE_FLAGS);
}

flag_option! {
    #[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TrunSampleCompositionOffset(pub VersionedSignedU32, TrunFlags, HAS_SAMPLE_COMPOSITION);
}

mp4_versioned_data! {
     #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
    pub struct TrunOffset {
        pub data_offset: TrunDataOffset,
        pub first_sample_flags: TrunFirstSampleFlags,
    }
}

mp4_versioned_data! {
     #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
    pub struct TrunEntry {
        pub sample_duration: TrunSampleDuration,
        pub sample_size: TrunSampleSize,
        pub sample_flags: TrunSampleFlags,
        pub sample_composition_time_offset: TrunSampleCompositionOffset
    }
}

full_box! {
    box (b"trun", Trun, TrunBox, TrunFlags)
    data {
        entries: Mp4VersionedOffsetArray<u32, TrunOffset, TrunEntry>
    }
}
