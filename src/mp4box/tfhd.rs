use std::hash::{Hash, Hasher};
use bitregions::bitregions;
use crate::{flag_option, full_box};
use crate::mp4box::trex::SampleFlags;

bitregions! {
    pub TfhdFlags u32 {
        HAS_BASE_DATA_OFFSET:           0b0000000000000000__0000_0000_0000_0001,
        HAS_SAMPLE_DESCRIPTION_INDEX:   0b0000000000000000__0000_0000_0000_0010,
        HAS_DEFAULT_SAMPLE_DURATION:    0b0000000000000000__0000_0000_0000_1000,
        HAS_DEFAULT_SAMPLE_SIZE:        0b0000000000000000__0000_0000_0001_0000,
        HAS_DEFAULT_SAMPLE_FLAGS:       0b0000000000000000__0000_0000_0010_0000,
        DURATION_IS_EMPTY:              0b0000000000000001__0000_0000_0000_0000,
        DEFAULT_BASE_IS_MOOF:           0b0000000000000010__0000_0000_0000_0000,
    }
}

impl Eq for TfhdFlags {}
impl Hash for TfhdFlags {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

flag_option! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TfhdDataOffset(pub u64, TfhdFlags, HAS_BASE_DATA_OFFSET);
}

flag_option! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TfhdSampleDescriptionIndex(pub u32, TfhdFlags, HAS_SAMPLE_DESCRIPTION_INDEX);
}

flag_option! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TfhdDefaultSampleDuration(pub u32, TfhdFlags, HAS_DEFAULT_SAMPLE_DURATION);
}

flag_option! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TfhdDefaultSampleSize(pub u32, TfhdFlags, HAS_DEFAULT_SAMPLE_SIZE);
}

flag_option! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TfhdDefaultSampleFlags(pub SampleFlags, TfhdFlags, HAS_DEFAULT_SAMPLE_FLAGS);
}

full_box! {
    box (b"tfhd", Tfhd, TfhdBox, @save flags: TfhdFlags)
    data {
        track_id: u32,
        base_data_offset: TfhdDataOffset,
        sample_description_index: TfhdSampleDescriptionIndex,
        default_sample_duration: TfhdDefaultSampleDuration,
        default_sample_size: TfhdDefaultSampleSize,
        default_sample_flags: TfhdDefaultSampleFlags,
    }
}
