use crate::base_box;
use crate::mp4box::co64::Co64Box;
use crate::mp4box::stco::StcoBox;
use crate::mp4box::stsc::StscBox;
use crate::mp4box::stsd::StsdBox;
use crate::mp4box::stts::SttsBox;

base_box! {
    box (b"stbl", Stbl, StblBox) children {
        stsd: StsdBox,
        stts: SttsBox,
        stsc: StscBox,
        stco: StcoBox,
        co64: Co64Box,
    }
}
