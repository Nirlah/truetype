//! The [OS/2 and Windows metrics][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/os2.htm

use {Result, Tape, Value};

/// OS/2 and Windows metrics.
#[derive(Clone, Copy, Debug)]
pub enum WindowsMetrics {
    /// Version 0.
    Version0(WindowsMetrics0),
    /// Version 1.
    Version1(WindowsMetrics1),
    /// Version 2.
    Version2(WindowsMetrics2),
    /// Version 3.
    Version3(WindowsMetrics3),
    /// Version 4.
    Version4(WindowsMetrics4),
    /// Version 5.
    Version5(WindowsMetrics5),
}

table! {
    #[doc = "OS/2 and Windows metrics of version 0."]
    #[derive(Copy)]
    pub WindowsMetrics0 {
        version               (u16           ), // version
        average_char_width    (i16           ), // xAvgCharWidth
        weight_class          (u16           ), // usWeightClass
        width_class           (u16           ), // usWidthClass
        type_flags            (TypeFlags     ), // fsType
        subscript_x_size      (i16           ), // ySubscriptXSize
        subscript_y_size      (i16           ), // ySubscriptYSize
        subscript_x_offset    (i16           ), // ySubscriptXOffset
        subscript_y_offset    (i16           ), // ySubscriptYOffset
        superscript_x_size    (i16           ), // ySuperscriptXSize
        superscript_y_size    (i16           ), // ySuperscriptYSize
        superscript_x_offset  (i16           ), // ySuperscriptXOffset
        superscript_y_offset  (i16           ), // ySuperscriptYOffset
        strikeout_size        (i16           ), // yStrikeoutSize
        strikeout_position    (i16           ), // yStrikeoutPosition
        family_class          (i16           ), // sFamilyClass
        panose                ([u8; 10]      ), // panose
        unicode_range1        (u32           ), // ulUnicodeRange1
        unicode_range2        (u32           ), // ulUnicodeRange2
        unicode_range3        (u32           ), // ulUnicodeRange3
        unicode_range4        (u32           ), // ulUnicodeRange4
        vendor_id             ([i8; 4]       ), // achVendID
        selection_flags       (SelectionFlags), // fsSelection
        first_char_index      (u16           ), // usFirstCharIndex
        last_char_index       (u16           ), // usLastCharIndex
        typographic_ascender  (i16           ), // sTypoAscender
        typographic_descender (i16           ), // sTypoDescender
        typographic_line_gap  (i16           ), // sTypoLineGap
        windows_ascender      (u16           ), // usWinAscent
        windows_descender     (u16           ), // usWinDescent
    }
}

table! {
    #[doc = "OS/2 and Windows metrics of version 1."]
    #[derive(Copy)]
    pub WindowsMetrics1 {
        version               (u16           ), // version
        average_char_width    (i16           ), // xAvgCharWidth
        weight_class          (u16           ), // usWeightClass
        width_class           (u16           ), // usWidthClass
        type_flags            (TypeFlags     ), // fsType
        subscript_x_size      (i16           ), // ySubscriptXSize
        subscript_y_size      (i16           ), // ySubscriptYSize
        subscript_x_offset    (i16           ), // ySubscriptXOffset
        subscript_y_offset    (i16           ), // ySubscriptYOffset
        superscript_x_size    (i16           ), // ySuperscriptXSize
        superscript_y_size    (i16           ), // ySuperscriptYSize
        superscript_x_offset  (i16           ), // ySuperscriptXOffset
        superscript_y_offset  (i16           ), // ySuperscriptYOffset
        strikeout_size        (i16           ), // yStrikeoutSize
        strikeout_position    (i16           ), // yStrikeoutPosition
        family_class          (i16           ), // sFamilyClass
        panose                ([u8; 10]      ), // panose
        unicode_range1        (u32           ), // ulUnicodeRange1
        unicode_range2        (u32           ), // ulUnicodeRange2
        unicode_range3        (u32           ), // ulUnicodeRange3
        unicode_range4        (u32           ), // ulUnicodeRange4
        vendor_id             ([i8; 4]       ), // achVendID
        selection_flags       (SelectionFlags), // fsSelection
        first_char_index      (u16           ), // usFirstCharIndex
        last_char_index       (u16           ), // usLastCharIndex
        typographic_ascender  (i16           ), // sTypoAscender
        typographic_descender (i16           ), // sTypoDescender
        typographic_line_gap  (i16           ), // sTypoLineGap
        windows_ascender      (u16           ), // usWinAscent
        windows_descender     (u16           ), // usWinDescent
        code_page_range1      (u32           ), // ulCodePageRange1
        code_page_range2      (u32           ), // ulCodePageRange2
    }
}

table! {
    #[doc = "OS/2 and Windows metrics of version 2."]
    #[derive(Copy)]
    pub WindowsMetrics2 {
        version               (u16           ), // version
        average_char_width    (i16           ), // xAvgCharWidth
        weight_class          (u16           ), // usWeightClass
        width_class           (u16           ), // usWidthClass
        type_flags            (TypeFlags     ), // fsType
        subscript_x_size      (i16           ), // ySubscriptXSize
        subscript_y_size      (i16           ), // ySubscriptYSize
        subscript_x_offset    (i16           ), // ySubscriptXOffset
        subscript_y_offset    (i16           ), // ySubscriptYOffset
        superscript_x_size    (i16           ), // ySuperscriptXSize
        superscript_y_size    (i16           ), // ySuperscriptYSize
        superscript_x_offset  (i16           ), // ySuperscriptXOffset
        superscript_y_offset  (i16           ), // ySuperscriptYOffset
        strikeout_size        (i16           ), // yStrikeoutSize
        strikeout_position    (i16           ), // yStrikeoutPosition
        family_class          (i16           ), // sFamilyClass
        panose                ([u8; 10]      ), // panose
        unicode_range1        (u32           ), // ulUnicodeRange1
        unicode_range2        (u32           ), // ulUnicodeRange2
        unicode_range3        (u32           ), // ulUnicodeRange3
        unicode_range4        (u32           ), // ulUnicodeRange4
        vendor_id             ([i8; 4]       ), // achVendID
        selection_flags       (SelectionFlags), // fsSelection
        first_char_index      (u16           ), // usFirstCharIndex
        last_char_index       (u16           ), // usLastCharIndex
        typographic_ascender  (i16           ), // sTypoAscender
        typographic_descender (i16           ), // sTypoDescender
        typographic_line_gap  (i16           ), // sTypoLineGap
        windows_ascender      (u16           ), // usWinAscent
        windows_descender     (u16           ), // usWinDescent
        code_page_range1      (u32           ), // ulCodePageRange1
        code_page_range2      (u32           ), // ulCodePageRange2
        x_height              (i16           ), // sxHeight
        cap_height            (i16           ), // sCapHeight
        default_char          (u16           ), // usDefaultChar
        break_char            (u16           ), // usBreakChar
        max_context           (u16           ), // usMaxContext
    }
}

/// OS/2 and Windows metrics of version 3.
pub type WindowsMetrics3 = WindowsMetrics2;

/// OS/2 and Windows metrics of version 4.
pub type WindowsMetrics4 = WindowsMetrics2;

table! {
    #[doc = "OS/2 and Windows metrics of version 5."]
    #[derive(Copy)]
    pub WindowsMetrics5 {
        version                  (u16           ), // version
        average_char_width       (i16           ), // xAvgCharWidth
        weight_class             (u16           ), // usWeightClass
        width_class              (u16           ), // usWidthClass
        type_flags               (TypeFlags     ), // fsType
        subscript_x_size         (i16           ), // ySubscriptXSize
        subscript_y_size         (i16           ), // ySubscriptYSize
        subscript_x_offset       (i16           ), // ySubscriptXOffset
        subscript_y_offset       (i16           ), // ySubscriptYOffset
        superscript_x_size       (i16           ), // ySuperscriptXSize
        superscript_y_size       (i16           ), // ySuperscriptYSize
        superscript_x_offset     (i16           ), // ySuperscriptXOffset
        superscript_y_offset     (i16           ), // ySuperscriptYOffset
        strikeout_size           (i16           ), // yStrikeoutSize
        strikeout_position       (i16           ), // yStrikeoutPosition
        family_class             (i16           ), // sFamilyClass
        panose                   ([u8; 10]      ), // panose
        unicode_range1           (u32           ), // ulUnicodeRange1
        unicode_range2           (u32           ), // ulUnicodeRange2
        unicode_range3           (u32           ), // ulUnicodeRange3
        unicode_range4           (u32           ), // ulUnicodeRange4
        vendor_id                ([i8; 4]       ), // achVendID
        selection_flags          (SelectionFlags), // fsSelection
        first_char_index         (u16           ), // usFirstCharIndex
        last_char_index          (u16           ), // usLastCharIndex
        typographic_ascender     (i16           ), // sTypoAscender
        typographic_descender    (i16           ), // sTypoDescender
        typographic_line_gap     (i16           ), // sTypoLineGap
        windows_ascender         (u16           ), // usWinAscent
        windows_descender        (u16           ), // usWinDescent
        code_page_range1         (u32           ), // ulCodePageRange1
        code_page_range2         (u32           ), // ulCodePageRange2
        x_height                 (i16           ), // sxHeight
        cap_height               (i16           ), // sCapHeight
        default_char             (u16           ), // usDefaultChar
        break_char               (u16           ), // usBreakChar
        max_context              (u16           ), // usMaxContext
        lower_optical_point_size (u16           ), // usLowerOpticalPointSize
        upper_optical_point_size (u16           ), // usUpperOpticalPointSize
    }
}

flags! {
    #[doc = "Type flags."]
    pub TypeFlags(u16) {
        0b1111_1100_0000_0000 => is_invalid,
    }
}

flags! {
    #[doc = "Font-selection flags."]
    pub SelectionFlags(u16) {
        0b1111_1100_0000_0000 => is_invalid,
    }
}

impl Value for WindowsMetrics {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            0 => WindowsMetrics::Version0(tape.take()?),
            1 => WindowsMetrics::Version1(tape.take()?),
            2 => WindowsMetrics::Version2(tape.take()?),
            3 => WindowsMetrics::Version3(tape.take()?),
            4 => WindowsMetrics::Version4(tape.take()?),
            5 => WindowsMetrics::Version5(tape.take()?),
            _ => raise!("found an unknown format of the OS/2 and Windows metrics"),
        })
    }
}
