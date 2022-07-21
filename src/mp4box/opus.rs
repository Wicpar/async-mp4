use crate::base_box;
use crate::mp4box::dops::DOpsBox;
use crate::types::sample::AudioSampleEntry;

base_box! {
    box (b"Opus", Opus, OpusBox) data {
        audio: AudioSampleEntry
    } children {
        dops: DOpsBox
    }
}
