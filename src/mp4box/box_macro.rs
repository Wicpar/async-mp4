

#[macro_export]
macro_rules! base_box {
    (box ($id:expr, $name:ident, $box:ident) $(data { $($data_name:ident: $data:ty),* })? $(children { $($child_name:ident: $child:tt),* $(,)*})?) => {
        pub type $box = $crate::mp4box::box_root::MP4Box<$name>;

        #[derive(Debug, Clone)]
        pub struct $name {
            $($(pub $data_name: $data),*)?
            $($(pub $child_name: base_box!(@type $child)),*)?
        }


        impl $crate::mp4box::box_trait::PartialBox for $name {
            type ParentData = ();
            type ThisData = ();

            fn byte_size(&self) -> usize {
                use $crate::mp4box::box_trait::IBox;
                $($(self.$data_name.byte_size() +)*)?
                $($(self.$child_name.byte_size() +)*)? 0
            }

            const ID: $crate::r#type::BoxType = $crate::r#type::BoxType::Id($crate::id::BoxId(*$id));
        }


        #[async_trait::async_trait]
        impl<R: $crate::bytes_read::ReadMp4> $crate::mp4box::box_trait::PartialBoxRead<R> for $name{
            async fn read_data(_: Self::ParentData, reader: &mut R) -> Result<Self, $crate::error::MP4Error> {
                Ok(Self {
                    $($($data_name: reader.read().await?,)*)?
                    $($($child_name: Default::default(),)*)?
                })
            }

            async fn read_child(&mut self, header: $crate::header::BoxHeader, reader: &mut R) -> Result<(), $crate::error::MP4Error> {
                use $crate::mp4box::box_trait::IBox;
                use $crate::mp4box::box_trait::BoxRead;
                match header.id {
                    $($(base_box!(@id $child) => base_box!(@read self.$child_name, header, reader, $child),)*)?
                    _ => {}
                }
                Ok(())
            }
        }

        #[async_trait::async_trait]
        impl<W: $crate::bytes_write::WriteMp4> $crate::mp4box::box_trait::PartialBoxWrite<W> for $name {

            async fn write_data(&self, writer: &mut W) -> Result<usize, $crate::error::MP4Error> {
                let mut count = 0;
                $($(count += self.$data_name.write(writer).await?;)*)?
                Ok(count)
            }


            async fn write_children(&self, writer: &mut W) -> Result<usize, $crate::error::MP4Error> {
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
    (box ($id:expr, $name:ident, $box:ident, $flag:ty) $(data { $($data_name:ident: $data:ty),* })? $(children { $($child_name:ident: $child:tt),* $(,)*})?) => {
        pub type $box = $crate::mp4box::box_root::MP4Box<$crate::mp4box::box_full::FullBox<$name>>;

        #[derive(Debug, Clone)]
        pub struct $name {
            $($(pub $data_name: $data),*)?
            $($(pub $child_name: base_box!(@type $child)),*)?
        }

        impl $crate::mp4box::box_full::FullBoxInfo<$flag> for $name {
            fn version(&self) -> u8 {
                full_box!(@max
                    $($(self.$data_name.required_version()),*)?
                )
            }

            fn flags(&self) -> $flag {
                $($(self.$data_name.required_flags() |)*)? <$flag>::default()
            }
        }

        impl $crate::mp4box::box_trait::PartialBox for $name {
            type ParentData = $crate::mp4box::box_full::FullBoxData<$flag>;
            type ThisData = ();

            fn byte_size(&self) -> usize {
                use $crate::mp4box::box_trait::IBox;
                $($(self.$data_name.byte_size() +)*)?
                $($(self.$child_name.byte_size() +)*)? 0
            }

            const ID: $crate::r#type::BoxType = $crate::r#type::BoxType::Id($crate::id::BoxId(*$id));
        }


        #[async_trait::async_trait]
        impl<R: $crate::bytes_read::ReadMp4> $crate::mp4box::box_trait::PartialBoxRead<R> for $name {
            async fn read_data(_: Self::ParentData, reader: &mut R) -> Result<Self, $crate::error::MP4Error> {
                Ok(Self {
                    $($($data_name: reader.read().await?,)*)?
                    $($($child_name: Default::default(),)*)?
                })
            }

            async fn read_child(&mut self, header: $crate::header::BoxHeader, reader: &mut R) -> Result<(), $crate::error::MP4Error> {
                use $crate::mp4box::box_trait::IBox;
                use $crate::mp4box::box_trait::BoxRead;
                match header.id {
                    $($(base_box!(@id $child) => base_box!(@read self.$child_name, header, reader, $child),)*)?
                    _ => {}
                }
                Ok(())
            }
        }

        #[async_trait::async_trait]
        impl<W: $crate::bytes_write::WriteMp4> $crate::mp4box::box_trait::PartialBoxWrite<W> for $name {

            async fn write_data<W: WriteMp4>(&self, writer: &mut W) -> Result<usize, MP4Error> {
                let mut count = 0;
                $($(count += self.$data_name.write(writer).await?;)*)?
                Ok(count)
            }

            async fn write_children(&self, writer: &mut W) -> Result<usize, $crate::error::MP4Error> {
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