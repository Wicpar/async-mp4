use crate::base_box;
use crate::mp4box::dinf::DinfBox;
use crate::mp4box::smhd::SmhdBox;
use crate::mp4box::stbl::StblBox;
use crate::mp4box::vmhd::VmhdBox;

base_box! {
    box (b"minf", Minf, MinfBox) children {
        vmhd: VmhdBox,
        smhd: SmhdBox,
        dinf: DinfBox,
        stbl: StblBox
    }
}
