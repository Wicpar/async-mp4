use crate::full_box;

full_box! {
    box (b"hdlr", Hdlr, HdlrBox, u32)
    data {
        _res1: u32,
        handler_type: [u8; 4],
        _res2: [u32; 3],
        name: String,
    }
}

impl Default for Hdlr {
    fn default() -> Self {
        Self {
            _res1: Default::default(),
            handler_type: Default::default(),
            _res2: Default::default(),
            name: Default::default()
        }
    }
}
