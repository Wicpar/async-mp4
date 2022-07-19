use crate::base_box;
use crate::mp4box::stsd::StsdBox;

base_box! {
    box (b"stbl", Stbl, StblBox) children {
        stsd: StsdBox
    }
}
