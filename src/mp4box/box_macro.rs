

#[macro_export]
macro_rules! base_box {
    (box ($id:expr, $name:ident, $box:ident) $(data { $($data_name:ident: $data:ty),* })? $(children { $($child_name:ident: $child:tt),* $(,)*})?) => {
        pub type $box = $crate::mp4box::box_root::MP4Box<$name>;

        #[derive(Debug, Clone)]
        pub struct $name {
            $($(pub $data_name: $data,)*)?
            $($(pub $child_name: base_box!(@type $child)),*)?
        }


        impl $crate::mp4box::box_trait::PartialBox for $name {
            type ParentData = ();
            type ThisData = ();

            fn byte_size(&self) -> usize {
                #![allow(unused_imports)]
                use $crate::mp4box::box_trait::IBox;
                $($(self.$data_name.byte_size() +)*)?
                $($(self.$child_name.byte_size() +)*)? 0
            }

            const ID: $crate::r#type::BoxType = $crate::r#type::BoxType::Id($crate::id::BoxId(*$id));
        }

        #[allow(unused_variables, unused_mut, dead_code)]
        #[async_trait::async_trait]
        impl<R: $crate::bytes_read::ReadMp4> $crate::mp4box::box_trait::PartialBoxRead<R> for $name{
            async fn read_data(_: Self::ParentData, reader: &mut R) -> Result<Self, $crate::error::MP4Error> {
                Ok(Self {
                    $($($data_name: reader.read().await?,)*)?
                    $($($child_name: Default::default(),)*)?
                })
            }

            async fn read_child(&mut self, header: $crate::header::BoxHeader, reader: &mut R) -> Result<(), $crate::error::MP4Error> {
                #![allow(unused_imports)]
                use $crate::mp4box::box_trait::IBox;
                use $crate::mp4box::box_trait::BoxRead;
                match header.id {
                    $($(base_box!(@id $child) => base_box!(@read self.$child_name, header, reader, $child),)*)?
                    _ => {}
                }
                Ok(())
            }
        }

        #[allow(unused_variables, unused_mut, dead_code)]
        #[async_trait::async_trait]
        impl<W: $crate::bytes_write::WriteMp4> $crate::mp4box::box_trait::PartialBoxWrite<W> for $name {

            async fn write_data(&self, writer: &mut W) -> Result<usize, $crate::error::MP4Error> {
                let mut count = 0;
                $($(count += self.$data_name.write(writer).await?;)*)?
                Ok(count)
            }


            async fn write_children(&self, writer: &mut W) -> Result<usize, $crate::error::MP4Error> {
                #![allow(unused_imports)]
                use $crate::mp4box::box_trait::BoxWrite;
                let mut count = 0;
                $($(count += self.$child_name.write(writer).await?;)*)?
                Ok(count)
            }
        }

    };

    (@type vec $child:ty) => {
        Vec<$child>
    };

    (@type $child:ty) => {
        Option<$child>
    };

    (@id $(vec)? $child:ty) => {
        <$child>::ID
    };

    (@read $child_name:expr, $header:ident, $reader:ident, vec $child:ty) => {
        $child_name.push(<$child>::read($header, $reader).await?)
    };

    (@read $child_name:expr, $header:ident, $reader:ident, $child:ty) => {
        $child_name = Some(<$child>::read($header, $reader).await?)
    };
}


#[macro_export]
macro_rules! full_box {
    (box ($id:expr, $name:ident, $box:ident, $(@save $flag_name:ident :)? $flag:ty) $(data { $($data_name:ident: $data:ty),* $(,)* })? $(children { $($child_name:ident: $child:tt),* $(,)*})?) => {
        pub type $box = $crate::mp4box::box_root::MP4Box<$crate::mp4box::box_full::FullBox<$name, $flag>>;

        #[derive(Debug, Clone, Eq, PartialEq, Hash)]
        pub struct $name {
            $($(pub $data_name: $data,)*)?
            $(pub $flag_name: $flag,)?
            $($(pub $child_name: base_box!(@type $child)),*)?
        }

        impl $crate::mp4box::box_full::FullBoxInfo for $name {
            type Flag = $flag;

            fn version(&self) -> u8 {
                #![allow(unused_imports)]
                use $crate::bytes_write::Mp4VersionedWritable;
                full_box!(@max
                    $($(Mp4VersionedWritable::<$flag>::required_version(&self.$data_name)),*)?
                )
            }

            fn flags(&self) -> $flag {
                #![allow(unused_imports)]
                use $crate::bytes_write::Mp4VersionedWritable;
                $($(Mp4VersionedWritable::<$flag>::required_flags(&self.$data_name) |)*)? $(self.$flag_name |)? <$flag>::default()
            }
        }

        impl $crate::mp4box::box_trait::PartialBox for $name {
            type ParentData = $crate::mp4box::box_full::FullBoxData<$flag>;
            type ThisData = ();

            fn byte_size(&self) -> usize {
                #![allow(unused_imports)]
                use $crate::mp4box::box_trait::IBox;
                use $crate::mp4box::box_full::FullBoxInfo;
                use $crate::bytes_write::Mp4VersionedWritable;
                let version = self.version();
                let flags = self.flags();
                $($(self.$data_name.versioned_byte_size(version, flags) +)*)?
                $($(self.$child_name.byte_size() +)*)? 0
            }

            const ID: $crate::r#type::BoxType = $crate::r#type::BoxType::Id($crate::id::BoxId(*$id));
        }

        #[allow(unused_variables, unused_mut, dead_code)]
        #[async_trait::async_trait]
        impl<R: $crate::bytes_read::ReadMp4> $crate::mp4box::box_trait::PartialBoxRead<R> for $name {

            async fn read_data(parent: Self::ParentData, reader: &mut R) -> Result<Self, $crate::error::MP4Error> {
                let version = parent.version;
                let flags = parent.flags;
                Ok(Self {
                    $($flag_name: flags,)?
                    $($($data_name: reader.versioned_read(version, flags).await?,)*)?
                    $($($child_name: Default::default(),)*)?
                })
            }

            async fn read_child(&mut self, header: $crate::header::BoxHeader, reader: &mut R) -> Result<(), $crate::error::MP4Error> {
                #![allow(unused_imports)]
                use $crate::mp4box::box_trait::IBox;
                use $crate::mp4box::box_trait::BoxRead;
                match header.id {
                    $($(base_box!(@id $child) => base_box!(@read self.$child_name, header, reader, $child),)*)?
                    _ => {}
                }
                Ok(())
            }
        }

        #[allow(unused_variables, unused_mut, dead_code)]
        #[async_trait::async_trait]
        impl<W: $crate::bytes_write::WriteMp4> $crate::mp4box::box_trait::PartialBoxWrite<W> for $name {

            async fn write_data(&self, writer: &mut W) -> Result<usize, MP4Error> {
                #![allow(unused_imports)]
                use $crate::bytes_write::Mp4VersionedWritable;
                use $crate::mp4box::box_full::FullBoxInfo;
                let version = self.version();
                let flags = self.flags();
                let mut count = 0;
                $($(count += self.$data_name.versioned_write(version, flags, writer).await?;)*)?
                Ok(count)
            }

            async fn write_children(&self, writer: &mut W) -> Result<usize, $crate::error::MP4Error> {
                #![allow(unused_imports)]
                use $crate::mp4box::box_trait::BoxWrite;
                let mut count = 0;
                $($(count += self.$child_name.write(writer).await?;)*)?
                Ok(count)
            }
        }

    };

    (@type vec $child:ty) => {
        Vec<$child>
    };

    (@type $child:ty) => {
        Option<$child>
    };

    (@id $(vec)? $child:ty) => {
        <$child>::ID
    };

    (@read $child_name:expr, $header:ident, $reader:ident, $version:ident, $flags:ident, vec $child:ty) => {
        $child_name.push(<$child>::read($header, $reader).await?)
    };

    (@read $child_name:expr, $header:ident, $reader:ident, $child:ty) => {
        $child_name = Some(<$child>::read($header, $reader).await?)
    };

    (@max $x:expr) => ( $x );
    (@max $x:expr, $($xs:expr),+) => {
        {
            use std::cmp::max;
            max($x, full_box!(@max $($xs),+ ))
        }
    };

    (@min $x:expr) => ( $x );
    (@min $x:expr, $($xs:expr),+) => {
        {
            use std::cmp::min;
            min($x, full_box!(@min $($xs),+ ))
        }
    };
}


#[macro_export]
macro_rules! default_flags {
    ($name:ident, $default:expr) => {
        #[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
        pub struct $name(pub u32);

        impl Default for VmhdFlags {
            fn default() -> Self {
                Self($default)
            }
        }

        impl From<u32> for VmhdFlags {
            fn from(value: u32) -> Self {
                Self(value)
            }
        }

        impl std::ops::BitOr for VmhdFlags {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl From<VmhdFlags> for u32 {
            fn from(value: VmhdFlags) -> Self {
                value.0
            }
        }
    }
}
