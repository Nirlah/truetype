use std::mem;

use Fixed;

/// A font-table tag.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Tag(pub u32);

impl From<Tag> for [u8; 4] {
    #[inline]
    fn from(tag: Tag) -> Self {
        unsafe { mem::transmute(u32::from_be(tag.0)) }
    }
}

impl From<[u8; 4]> for Tag {
    #[inline]
    fn from(bytes: [u8; 4]) -> Self {
        Tag(u32::from_be(unsafe { mem::transmute(bytes) }))
    }
}

impl<'l> From<&'l [u8; 4]> for Tag {
    #[inline(always)]
    fn from(bytes: &'l [u8; 4]) -> Self {
        (*bytes).into()
    }
}

impl From<Fixed> for Tag {
    #[inline(always)]
    fn from(fixed: Fixed) -> Self {
        Tag(fixed.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Tag;

    #[test]
    fn from() {
        assert_eq!(Tag::from(b"true"), Tag(0x74727565));
    }
}