//! Compound data types.

macro_rules! itemize(($($chunk:item)*) => ($($chunk)*));

macro_rules! table {
    ($(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($($kind:tt)+) $(|$($argument:ident),+| $body:block)*,)+
    }) => (
        define_table! { $(#[$attribute])* pub $structure { $($field ($($kind)+),)+ } }
        read_table! { pub $structure { $($field ($($kind)+) $(|$($argument),+| $body)*,)+ } }
    );
}

macro_rules! define_table {
    ($(#[$attribute:meta])* pub $structure:ident { $($field:ident ($kind:ty),)+ }) => (itemize! {
        $(#[$attribute])*
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $structure { $(pub $field: $kind,)+ }
    });
}

macro_rules! read_table {
    (pub $structure:ident {
        $($field:ident ($($kind:tt)+) $(|$($argument:ident),+| $body:block)*,)+
    }) => (
        impl ::tape::Value for $structure {
            fn read<T: ::tape::Tape>(tape: &mut T) -> ::Result<Self> {
                let mut table = $structure::default();
                $(
                    table.$field = read_field!($structure, tape, table, [$($kind)+]
                                               $(|$($argument),+| $body)*);
                )+
                Ok(table)
            }
        }
    );
}

macro_rules! read_field(
    ($structure:ident, $tape:ident, $table:ident,
     [$kind:ty] |$pipe:ident, $chair:ident| $body:block) => ({
        #[inline(always)]
        #[allow(unused_variables)]
        fn read<T: ::tape::Tape>($pipe: &mut T, $chair: &$structure) -> ::Result<$kind> $body
        try!(read($tape, &$table))
    });
    ($structure:ident, $tape:expr, $table:expr, [$kind:ty]) => ({
        try!(::tape::Value::read($tape))
    });
);

macro_rules! read_array(
    (@common $tape:ident, $count:expr) => ({
        let count = $count as usize;
        let mut values = Vec::with_capacity(count);
        values.set_len(count);
        if try!(::std::io::Read::read($tape, &mut values)) != count {
            return raise!("failed to read as much as needed");
        }
        let mut array: [u8; $count] = ::std::mem::uninitialized();
        for (destination, source) in array.iter_mut().zip(values.iter()) {
            *destination = *source;
        }
        array
    });
    ($tape:ident, $count:expr, i8) => (unsafe {
        Ok(::std::mem::transmute(read_array!(@common $tape, $count)))
    });
    ($tape:ident, $count:expr, u8) => (unsafe {
        Ok(read_array!(@common $tape, $count))
    });
);

macro_rules! read_vector(
    ($tape:ident, $count:expr, u8) => (unsafe {
        let count = $count as usize;
        let mut values = Vec::with_capacity(count);
        values.set_len(count);
        if try!(::std::io::Read::read($tape, &mut values)) != count {
            return raise!("failed to read as much as needed");
        }
        Ok(values)
    });
    ($tape:ident, $count:expr) => ({
        let count = $count as usize;
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(try!(::tape::Value::read($tape)));
        }
        Ok(values)
    });
);

mod char_mapping;
mod font_header;
mod horizontal_header;
mod horizontal_metrics;
mod maximum_profile;
mod naming_table;
mod offset_table;
mod postscript_info;
mod windows_metrics;

pub use self::char_mapping::{CharMapping, CharMappingHeader, CharMappingRecord};
pub use self::char_mapping::{CharMappingEncoding, CharMappingEncoding4, CharMappingEncoding6};
pub use self::font_header::FontHeader;
pub use self::horizontal_header::HorizontalHeader;
pub use self::horizontal_metrics::{HorizontalMetrics, LongHorizontalMetric};
pub use self::maximum_profile::{MaximumProfile, MaximumProfile05, MaximumProfile10};
pub use self::naming_table::{NameRecord, LanguageTagRecord};
pub use self::naming_table::{NamingTable, NamingTable0, NamingTable1};
pub use self::offset_table::{OffsetTable, OffsetTableHeader, OffsetTableRecord};
pub use self::postscript_info::{PostScriptInfo, PostScriptInfo10, PostScriptInfo30};
pub use self::windows_metrics::{WindowsMetrics, WindowsMetrics3, WindowsMetrics5};
