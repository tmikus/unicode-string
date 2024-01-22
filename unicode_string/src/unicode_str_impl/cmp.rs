use crate::{unicode_str};

impl PartialEq for unicode_str {
    #[inline]
    fn eq(&self, other: &unicode_str) -> bool {
        self.chars == other.chars
    }
    #[inline]
    fn ne(&self, other: &unicode_str) -> bool {
        !(*self).eq(other)
    }
}

impl Eq for unicode_str {}

/// Implements comparison operations on strings.
///
/// Strings are compared [lexicographically](Ord#lexicographical-comparison) by their byte values. This compares Unicode code
/// points based on their positions in the code charts. This is not necessarily the same as
/// "alphabetical" order, which varies by language and locale. Comparing strings according to
/// culturally-accepted standards requires locale-specific data that is outside the scope of
/// the `str` type.
impl PartialOrd for unicode_str {
    #[inline]
    fn partial_cmp(&self, other: &unicode_str) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for unicode_str {
    #[inline]
    fn cmp(&self, other: &unicode_str) -> std::cmp::Ordering {
        self.chars.cmp(&other.chars)
    }
}

