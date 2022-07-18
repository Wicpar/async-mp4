use crate::base_box;
use crate::mp4box::mdhd::MdhdBox;

base_box! {
    box (b"mdia", Mdia, MdiaBox)
    children {
        mdhd: MdhdBox
    }
}
