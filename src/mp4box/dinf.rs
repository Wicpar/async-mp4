use crate::base_box;
use crate::mp4box::dref::{DrefBox};

base_box! {
    box (b"dinf", Dinf, DinfBox) children {
        dref: DrefBox
    }
}

impl Default for Dinf {
    fn default() -> Self {
        Self {
            dref: Some(DrefBox::default())
        }
    }
}
