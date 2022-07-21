use crate::full_box;
use crate::types::versioned_u32_u64::VersionedU32U64;

full_box! {
    box (b"tfdt", Tfdt, TfdtBox, u32)
    data {
        base_media_decode_time: VersionedU32U64,
    }
}
