use crate::{base_box};
use crate::types::sample::VisualSampleEntry;

base_box! {
    box (b"avc1", Avc1, Avc1Box) data {
        visual_sample_entry: VisualSampleEntry
    } children {

    }
}
