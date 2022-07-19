use crate::full_box;

full_box! {
    box (b"urn ", Urn, UrnBox, @save flags: u32) data {
        name: String,
        location: String
    }
}

impl Default for Urn {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            location: "".to_string(),
            flags: 1
        }
    }
}
