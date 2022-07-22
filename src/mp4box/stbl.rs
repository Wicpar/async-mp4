use crate::base_box;
use crate::mp4box::co64::Co64Box;
use crate::mp4box::stco::StcoBox;
use crate::mp4box::stsc::StscBox;
use crate::mp4box::stsd::StsdBox;
use crate::mp4box::stts::SttsBox;
use crate::mp4box::stsz::StszBox;

base_box! {
    box (b"stbl", Stbl, StblBox) children {
        co64: Co64Box,
        stco: StcoBox,
        stsc: StscBox,
        stsd: StsdBox,
        stsz: StszBox,
        stts: SttsBox,
    }
}
