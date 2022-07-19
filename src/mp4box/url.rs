use crate::full_box;


full_box! {
    box (b"url ", Url, UrlBox, @save flags: u32) data {
        location: String
    }
}

impl Default for Url {
    fn default() -> Self {
        Self {
            location: "".to_string(),
            flags: 1
        }
    }
}
