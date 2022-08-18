use std::hash::{Hash, Hasher};
use bitregions::bitregions;
use crate::{flag_option, full_box, mp4_versioned_data};
use crate::types::versioned_signed_int::VersionedSignedU32;
use crate::mp4box::trex::SampleFlags;
use crate::types::array::{Mp4VersionedOffsetArray};

bitregions! {
    pub TrunFlags u32 {
        HAS_DATA_OFFSET:                0b0000000000000000__0000_0000_0000_0001,
        HAS_FIRST_SAMPLE_FLAGS:         0b0000000000000000__0000_0000_0000_0100,
        HAS_SAMPLE_DURATION:            0b0000000000000000__0000_0001_0000_0000,
        HAS_SAMPLE_SIZE:                0b0000000000000000__0000_0010_0000_0000,
        HAS_SAMPLE_FLAGS:               0b0000000000000000__0000_0100_0000_0000,
        HAS_SAMPLE_COMPOSITION:         0b0000000000000000__0000_1000_0000_0000,
    }
}

impl Eq for TrunFlags {}
impl Hash for TrunFlags {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

flag_option! {
    #[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TrunDataOffset(pub i32, TrunFlags, HAS_DATA_OFFSET);
}

flag_option! {
    #[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TrunFirstSampleFlags(pub SampleFlags, TrunFlags, HAS_FIRST_SAMPLE_FLAGS);
}

flag_option! {
    #[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TrunSampleDuration(pub u32, TrunFlags, HAS_SAMPLE_DURATION);
}

flag_option! {
    #[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TrunSampleSize(pub u32, TrunFlags, HAS_SAMPLE_SIZE);
}

flag_option! {
    #[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TrunSampleFlags(pub SampleFlags, TrunFlags, HAS_SAMPLE_FLAGS);
}

flag_option! {
    #[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TrunSampleCompositionOffset(pub VersionedSignedU32, TrunFlags, HAS_SAMPLE_COMPOSITION);
}

mp4_versioned_data! {
     #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
    pub struct TrunOffset {
        pub data_offset: TrunDataOffset,
        pub first_sample_flags: TrunFirstSampleFlags,
    }
}

mp4_versioned_data! {
     #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
    pub struct TrunEntry {
        pub sample_duration: TrunSampleDuration,
        pub sample_size: TrunSampleSize,
        pub sample_flags: TrunSampleFlags,
        pub sample_composition_time_offset: TrunSampleCompositionOffset
    }
}

full_box! {
    box (b"trun", Trun, TrunBox, TrunFlags)
    data {
        entries: Mp4VersionedOffsetArray<u32, TrunOffset, TrunEntry>
    }
}

impl Default for Trun {
    fn default() -> Self {
        Self {
            entries: Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::bytes_read::Mp4Readable;
    use crate::error::MP4Error;
    use crate::header::BoxHeader;
    use crate::mp4box::box_trait::{BoxRead, BoxWrite, IBox};
    use crate::mp4box::trun::{Trun, TrunBox, TrunDataOffset, TrunEntry, TrunOffset, TrunSampleSize};
    use crate::types::array::Mp4VersionedOffsetArray;
    use crate::mp4box::trex::SampleFlags;

    #[test]
    pub fn test_rebuild() -> Result<(), MP4Error> {
        futures::executor::block_on(async {
            let base: TrunBox = Trun {
                entries: Mp4VersionedOffsetArray {
                    data: vec![TrunEntry {
                        sample_duration: 32u32.into(),
                        sample_size: TrunSampleSize::from(100000u32),
                        sample_flags: SampleFlags::from(37814272).into(),
                        sample_composition_time_offset: Default::default()
                    }],
                    offset: TrunOffset {
                        data_offset: TrunDataOffset(Some(100)),
                        first_sample_flags: Default::default()
                    },
                    _p: Default::default()
                }
            }.into();
            let mut buf = vec![];
            let mut cursor = std::io::Cursor::new(&mut buf);
            let pos = base.write(&mut cursor)?;
            assert_eq!(pos, base.byte_size());
            assert_eq!(pos as u64, cursor.position());
            let mut cursor = futures::io::Cursor::new(&mut buf);
            let header = BoxHeader::read(&mut cursor).await?;
            assert_eq!(header.id, TrunBox::ID);
            let new = TrunBox::read(header, &mut cursor).await?;
            assert_eq!(base, new);
            Ok(())
        })
    }

}
