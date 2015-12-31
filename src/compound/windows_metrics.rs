use Result;
use tape::{Tape, Value};

/// OS/2 and Windows metrics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WindowsMetrics {
    /// Version 3.
    Version3(WindowsMetrics3),
    /// Version 5.
    Version5(WindowsMetrics5),
}

table! {
    #[doc = "OS/2 and Windows metrics of version 3."]
    pub WindowsMetrics3 {
        version              (u16),
        average_char_width   (i16), // xAvgCharWidth
        weight_class         (u16), // usWeightClass
        width_class          (u16), // usWidthClass
        type_flags           (u16), // fsType
        subscript_x_size     (i16), // ySubscriptXSize
        subscript_y_size     (i16), // ySubscriptYSize
        subscript_x_offset   (i16), // ySubscriptXOffset
        subscript_y_offset   (i16), // ySubscriptYOffset
        superscript_x_size   (i16), // ySuperscriptXSize
        superscript_y_size   (i16), // ySuperscriptYSize
        superscript_x_offset (i16), // ySuperscriptXOffset
        superscript_y_offset (i16), // ySuperscriptYOffset
        strikeout_size       (i16), // yStrikeoutSize
        strikeout_position   (i16), // yStrikeoutPosition
        family_class         (i16), // sFamilyClass

        panose ([u8; 10]) |tape, this| {
            read_array!(tape, 10, u8)
        },

        unicode_range1 (u32), // ulUnicodeRange1
        unicode_range2 (u32), // ulUnicodeRange2
        unicode_range3 (u32), // ulUnicodeRange3
        unicode_range4 (u32), // ulUnicodeRange4

        vendor_id ([i8; 4]) |tape, this| { // achVendID
            read_array!(tape, 4, i8)
        },

        selection_flags       (u16), // fsSelection
        first_char_index      (u16), // usFirstCharIndex
        last_char_index       (u16), // usLastCharIndex
        typographic_ascender  (i16), // sTypoAscender
        typographic_descender (i16), // sTypoDescender
        typographic_line_gap  (i16), // sTypoLineGap
        windows_ascender      (u16), // usWinAscent
        windows_descender     (u16), // usWinDescent
        code_page_range1      (u32), // ulCodePageRange1
        code_page_range2      (u32), // ulCodePageRange2
        x_height              (i16), // sxHeight
        cap_height            (i16), // sCapHeight
        default_char          (u16), // usDefaultChar
        break_char            (u16), // usBreakChar
        max_context           (u16), // usMaxContext
    }
}

table! {
    #[doc = "OS/2 and Windows metrics of version 5."]
    pub WindowsMetrics5 {
        version              (u16),
        average_char_width   (i16), // xAvgCharWidth
        weight_class         (u16), // usWeightClass
        width_class          (u16), // usWidthClass
        type_flags           (u16), // fsType
        subscript_x_size     (i16), // ySubscriptXSize
        subscript_y_size     (i16), // ySubscriptYSize
        subscript_x_offset   (i16), // ySubscriptXOffset
        subscript_y_offset   (i16), // ySubscriptYOffset
        superscript_x_size   (i16), // ySuperscriptXSize
        superscript_y_size   (i16), // ySuperscriptYSize
        superscript_x_offset (i16), // ySuperscriptXOffset
        superscript_y_offset (i16), // ySuperscriptYOffset
        strikeout_size       (i16), // yStrikeoutSize
        strikeout_position   (i16), // yStrikeoutPosition
        family_class         (i16), // sFamilyClass

        panose ([u8; 10]) |tape, this| {
            read_array!(tape, 10, u8)
        },

        unicode_range1 (u32), // ulUnicodeRange1
        unicode_range2 (u32), // ulUnicodeRange2
        unicode_range3 (u32), // ulUnicodeRange3
        unicode_range4 (u32), // ulUnicodeRange4

        vendor_id ([i8; 4]) |tape, this| { // achVendID
            read_array!(tape, 4, i8)
        },

        selection_flags          (u16), // fsSelection
        first_char_index         (u16), // usFirstCharIndex
        last_char_index          (u16), // usLastCharIndex
        typographic_ascender     (i16), // sTypoAscender
        typographic_descender    (i16), // sTypoDescender
        typographic_line_gap     (i16), // sTypoLineGap
        windows_ascender         (u16), // usWinAscent
        windows_descender        (u16), // usWinDescent
        code_page_range1         (u32), // ulCodePageRange1
        code_page_range2         (u32), // ulCodePageRange2
        x_height                 (i16), // sxHeight
        cap_height               (i16), // sCapHeight
        default_char             (u16), // usDefaultChar
        break_char               (u16), // usBreakChar
        max_context              (u16), // usMaxContext
        lower_optical_point_size (u16), // usLowerOpticalPointSize
        upper_optical_point_size (u16), // usUpperOpticalPointSize
    }
}

impl Value for WindowsMetrics {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            3 => WindowsMetrics::Version3(try!(Value::read(tape))),
            5 => WindowsMetrics::Version5(try!(Value::read(tape))),
            _ => raise!("the format of the OS/2 and Windows metrics is not supported"),
        })
    }
}
