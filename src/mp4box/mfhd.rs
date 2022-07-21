use crate::full_box;

full_box! {
    box (b"mfhd", Mfhd, MfhdBox, u32)
    data {
        sequence_number: u32
    }
}
