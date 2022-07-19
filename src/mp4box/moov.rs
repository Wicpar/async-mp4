use crate::base_box;
use crate::mp4box::mvhd::MvhdBox;
use crate::mp4box::mvex::MvexBox;
use crate::mp4box::trak::TrakBox;

base_box! {
    box (b"moov", Moov, MoovBox) children {
        mvhd: MvhdBox,
        mvex: MvexBox,
        traks: vec TrakBox
    }
}
