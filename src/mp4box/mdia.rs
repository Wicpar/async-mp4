use crate::base_box;
use crate::mp4box::hdlr::HdlrBox;
use crate::mp4box::mdhd::MdhdBox;
use crate::mp4box::minf::MinfBox;

base_box! {
    box (b"mdia", Mdia, MdiaBox)
    children {
        mdhd: MdhdBox,
        hdlr: HdlrBox,
        minf: MinfBox
    }
}
