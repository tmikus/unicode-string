use crate::{unicode_str, UnicodeString};
use std::ops;

impl ops::Index<ops::Range<usize>> for UnicodeString {
    type Output = unicode_str;

    #[inline]
    fn index(&self, index: ops::Range<usize>) -> &unicode_str {
        &self[..][index]
    }
}
impl ops::Index<ops::RangeTo<usize>> for UnicodeString {
    type Output = unicode_str;

    #[inline]
    fn index(&self, index: ops::RangeTo<usize>) -> &unicode_str {
        &self[..][index]
    }
}
impl ops::Index<ops::RangeFrom<usize>> for UnicodeString {
    type Output = unicode_str;

    #[inline]
    fn index(&self, index: ops::RangeFrom<usize>) -> &unicode_str {
        &self[..][index]
    }
}
impl ops::Index<ops::RangeFull> for UnicodeString {
    type Output = unicode_str;

    #[inline]
    fn index(&self, _index: ops::RangeFull) -> &unicode_str {
        &self
    }
}
impl ops::Index<ops::RangeInclusive<usize>> for UnicodeString {
    type Output = unicode_str;

    #[inline]
    fn index(&self, index: ops::RangeInclusive<usize>) -> &unicode_str {
        ops::Index::index(&**self, index)
    }
}
impl ops::Index<ops::RangeToInclusive<usize>> for UnicodeString {
    type Output = unicode_str;

    #[inline]
    fn index(&self, index: ops::RangeToInclusive<usize>) -> &unicode_str {
        ops::Index::index(&**self, index)
    }
}

impl ops::IndexMut<ops::Range<usize>> for UnicodeString {
    #[inline]
    fn index_mut(&mut self, index: ops::Range<usize>) -> &mut unicode_str {
        &mut self[..][index]
    }
}
impl ops::IndexMut<ops::RangeTo<usize>> for UnicodeString {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeTo<usize>) -> &mut unicode_str {
        &mut self[..][index]
    }
}
impl ops::IndexMut<ops::RangeFrom<usize>> for UnicodeString {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeFrom<usize>) -> &mut unicode_str {
        &mut self[..][index]
    }
}
impl ops::IndexMut<ops::RangeFull> for UnicodeString {
    #[inline]
    fn index_mut(&mut self, _index: ops::RangeFull) -> &mut unicode_str {
        unicode_str::from_chars_mut(&mut *self.vec)
    }
}
impl ops::IndexMut<ops::RangeInclusive<usize>> for UnicodeString {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeInclusive<usize>) -> &mut unicode_str {
        ops::IndexMut::index_mut(&mut **self, index)
    }
}
impl ops::IndexMut<ops::RangeToInclusive<usize>> for UnicodeString {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeToInclusive<usize>) -> &mut unicode_str {
        ops::IndexMut::index_mut(&mut **self, index)
    }
}
