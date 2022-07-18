use crate::base_box;
use crate::mp4box::mdia::MdiaBox;
use crate::mp4box::tkhd::TkhdBox;

base_box! {
    box (b"trak", Trak, TrakBox) children {
        tkhd: TkhdBox,
        mdia: MdiaBox,
    }
}