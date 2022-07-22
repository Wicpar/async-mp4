use crate::base_box;
use crate::mp4box::trex::TrexBox;

base_box! {
    box (b"mvex", Mvex, MvexBox) children {
        trex: vec TrexBox
    }
}
