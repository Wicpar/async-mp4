use crate::base_box;
use crate::mp4box::mfhd::MfhdBox;
use crate::mp4box::traf::TrafBox;

base_box! {
    box (b"moof", Moof, MoofBox) children {
        mfhd: MfhdBox,
        trafs: vec TrafBox
    }
}
