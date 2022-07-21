use crate::base_box;
use crate::mp4box::tfdt::TfdtBox;
use crate::mp4box::tfhd::TfhdBox;
use crate::mp4box::trun::TrunBox;

base_box! {
    box (b"traf", Traf, TrafBox) children {
        tfhd: TfhdBox,
        tfdt: TfdtBox,
        truns: vec TrunBox
    }
}
