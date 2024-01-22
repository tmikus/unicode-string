use std::{ops, ptr};
use std::slice::SliceIndex;
use crate::unicode_str;

impl<I> ops::Index<I> for unicode_str
    where
        I: SliceIndex<unicode_str>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &I::Output {
        index.index(self)
    }
}

impl<I> ops::IndexMut<I> for unicode_str
    where
        I: SliceIndex<unicode_str>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut I::Output {
        index.index_mut(self)
    }
}

#[inline(never)]
#[cold]
#[track_caller]
const fn str_index_overflow_fail() -> ! {
    panic!("attempted to index str up to maximum usize");
}

/// Implements substring slicing with syntax `&self[..]` or `&mut self[..]`.
///
/// Returns a slice of the whole string, i.e., returns `&self` or `&mut
/// self`. Equivalent to `&self[0 .. len]` or `&mut self[0 .. len]`. Unlike
/// other indexing operations, this can never panic.
///
/// This operation is *O*(1).
///
/// Prior to 1.20.0, these indexing operations were still supported by
/// direct implementation of `Index` and `IndexMut`.
///
/// Equivalent to `&self[0 .. len]` or `&mut self[0 .. len]`.
unsafe impl const SliceIndex<unicode_str> for ops::RangeFull {
    type Output = unicode_str;
    #[inline]
    fn get(self, slice: &unicode_str) -> Option<&Self::Output> {
        Some(slice)
    }
    #[inline]
    fn get_mut(self, slice: &mut unicode_str) -> Option<&mut Self::Output> {
        Some(slice)
    }
    #[inline]
    unsafe fn get_unchecked(self, slice: *const unicode_str) -> *const Self::Output {
        slice
    }
    #[inline]
    unsafe fn get_unchecked_mut(self, slice: *mut unicode_str) -> *mut Self::Output {
        slice
    }
    #[inline]
    fn index(self, slice: &unicode_str) -> &Self::Output {
        slice
    }
    #[inline]
    fn index_mut(self, slice: &mut unicode_str) -> &mut Self::Output {
        slice
    }
}

/// Implements substring slicing with syntax `&self[begin .. end]` or `&mut
/// self[begin .. end]`.
///
/// Returns a slice of the given string from the byte range
/// [`begin`, `end`).
///
/// This operation is *O*(1).
///
/// Prior to 1.20.0, these indexing operations were still supported by
/// direct implementation of `Index` and `IndexMut`.
///
/// # Panics
///
/// Panics if `begin` or `end` does not point to the starting byte offset of
/// a character (as defined by `is_char_boundary`), if `begin > end`, or if
/// `end > len`.
///
/// # Examples
///
/// ```
/// use unicode_string::ustr;
///
/// let s = ustr!("Löwe 老虎 Léopard");
/// assert_eq!(&s[0 .. 1], ustr!("L"));
///
/// assert_eq!(&s[1 .. 6], ustr!("öwe 老"));
///
/// // these will panic:
/// // byte 2 lies within `ö`:
/// // &s[2 ..3];
///
/// // byte 8 lies within `老`
/// // &s[1 .. 8];
///
/// // byte 100 is outside the string
/// // &s[3 .. 100];
/// ```
unsafe impl const SliceIndex<unicode_str> for ops::Range<usize> {
    type Output = unicode_str;

    #[inline]
    fn get(self, slice: &unicode_str) -> Option<&Self::Output> {
        if self.start <= self.end
        {
            // SAFETY: just checked that `start` and `end` are on a char boundary,
            // and we are passing in a safe reference, so the return value will also be one.
            // We also checked char boundaries, so this is valid UTF-8.
            Some(unsafe { &*self.get_unchecked(slice) })
        } else {
            None
        }
    }
    #[inline]
    fn get_mut(self, slice: &mut unicode_str) -> Option<&mut Self::Output> {
        if self.start <= self.end
        {
            // SAFETY: just checked that `start` and `end` are on a char boundary.
            // We know the pointer is unique because we got it from `slice`.
            Some(unsafe { &mut *self.get_unchecked_mut(slice) })
        } else {
            None
        }
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: *const unicode_str) -> *const Self::Output {
        let slice =
            core::slice::from_raw_parts((*slice).chars.as_ptr(), (*slice).chars.len()).as_ptr();
        // SAFETY: the caller guarantees that `self` is in bounds of `slice`
        // which satisfies all the conditions for `add`.
        let ptr = slice.add(self.start);
        let len = self.end - self.start;
        ptr::slice_from_raw_parts(ptr, len) as *const unicode_str
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: *mut unicode_str) -> *mut Self::Output {
        // SAFETY: see comments for `get_unchecked`.
        let slice =
            core::slice::from_raw_parts_mut((*slice).chars.as_mut_ptr(), (*slice).chars.len());
        let ptr = slice.as_mut_ptr().add(self.start);
        let len = self.end - self.start;
        ptr::slice_from_raw_parts_mut(ptr, len) as *mut unicode_str
    }
    #[inline]
    fn index(self, slice: &unicode_str) -> &Self::Output {
        match self.get(slice) {
            Some(s) => s,
            None => slice_error_fail(),
        }
    }
    #[inline]
    fn index_mut(self, slice: &mut unicode_str) -> &mut Self::Output {
        if self.start <= self.end
        {
            // SAFETY: just checked that `start` and `end` are on a char boundary,
            // and we are passing in a safe reference, so the return value will also be one.
            unsafe { &mut *self.get_unchecked_mut(slice) }
        } else {
            slice_error_fail()
        }
    }
}

/// Implements substring slicing with syntax `&self[.. end]` or `&mut
/// self[.. end]`.
///
/// Returns a slice of the given string from the byte range \[0, `end`).
/// Equivalent to `&self[0 .. end]` or `&mut self[0 .. end]`.
///
/// This operation is *O*(1).
///
/// Prior to 1.20.0, these indexing operations were still supported by
/// direct implementation of `Index` and `IndexMut`.
///
/// # Panics
///
/// Panics if `end` does not point to the starting byte offset of a
/// character (as defined by `is_char_boundary`), or if `end > len`.
unsafe impl const SliceIndex<unicode_str> for ops::RangeTo<usize> {
    type Output = unicode_str;

    #[inline]
    fn get(self, slice: &unicode_str) -> Option<&Self::Output> {
        Some(unsafe { &*self.get_unchecked(slice) })
    }
    #[inline]
    fn get_mut(self, slice: &mut unicode_str) -> Option<&mut Self::Output> {
        Some(unsafe { &mut *self.get_unchecked_mut(slice) })
    }
    #[inline]
    unsafe fn get_unchecked(self, slice: *const unicode_str) -> *const Self::Output {
        ptr::slice_from_raw_parts((*slice).chars.as_ptr(), self.end) as *const unicode_str
    }
    #[inline]
    unsafe fn get_unchecked_mut(self, slice: *mut unicode_str) -> *mut Self::Output {
        ptr::slice_from_raw_parts_mut((*slice).chars.as_mut_ptr(), self.end) as *mut unicode_str
    }
    #[inline]
    fn index(self, slice: &unicode_str) -> &Self::Output {
        match self.get(slice) {
            Some(s) => s,
            None => slice_error_fail(),
        }
    }
    #[inline]
    fn index_mut(self, slice: &mut unicode_str) -> &mut Self::Output {
        unsafe { &mut *self.get_unchecked_mut(slice) }
    }
}

/// Implements substring slicing with syntax `&self[begin ..]` or `&mut
/// self[begin ..]`.
///
/// Returns a slice of the given string from the byte range \[`begin`, `len`).
/// Equivalent to `&self[begin .. len]` or `&mut self[begin .. len]`.
///
/// This operation is *O*(1).
///
/// Prior to 1.20.0, these indexing operations were still supported by
/// direct implementation of `Index` and `IndexMut`.
///
/// # Panics
///
/// Panics if `begin` does not point to the starting byte offset of
/// a character (as defined by `is_char_boundary`), or if `begin > len`.
unsafe impl const SliceIndex<unicode_str> for ops::RangeFrom<usize> {
    type Output = unicode_str;

    #[inline]
    fn get(self, slice: &unicode_str) -> Option<&Self::Output> {
        Some(unsafe { &*self.get_unchecked(slice) })
    }
    #[inline]
    fn get_mut(self, slice: &mut unicode_str) -> Option<&mut Self::Output> {
        Some(unsafe { &mut *self.get_unchecked_mut(slice) })
    }
    #[inline]
    unsafe fn get_unchecked(self, slice: *const unicode_str) -> *const Self::Output {
        let slice_data = (*slice).chars.as_ptr();
        // SAFETY: the caller guarantees that `self` is in bounds of `slice`
        // which satisfies all the conditions for `add`.
        let ptr = unsafe { slice_data.add(self.start) };
        let len = (*slice).chars.len() - self.start;
        ptr::slice_from_raw_parts(ptr, len) as *const unicode_str
    }
    #[inline]
    unsafe fn get_unchecked_mut(self, slice: *mut unicode_str) -> *mut Self::Output {
        let slice_data = (*slice).chars.as_mut_ptr();
        // SAFETY: identical to `get_unchecked`.
        let ptr = unsafe { slice_data.add(self.start) };
        let len = (*slice).chars.len() - self.start;
        ptr::slice_from_raw_parts_mut(ptr, len) as *mut unicode_str
    }
    #[inline]
    fn index(self, slice: &unicode_str) -> &Self::Output {
        match self.get(slice) {
            Some(s) => s,
            None => slice_error_fail(),
        }
    }
    #[inline]
    fn index_mut(self, slice: &mut unicode_str) -> &mut Self::Output {
        unsafe { &mut *self.get_unchecked_mut(slice) }
    }
}

/// Implements substring slicing with syntax `&self[begin ..= end]` or `&mut
/// self[begin ..= end]`.
///
/// Returns a slice of the given string from the byte range
/// [`begin`, `end`]. Equivalent to `&self [begin .. end + 1]` or `&mut
/// self[begin .. end + 1]`, except if `end` has the maximum value for
/// `usize`.
///
/// This operation is *O*(1).
///
/// # Panics
///
/// Panics if `begin` does not point to the starting byte offset of
/// a character (as defined by `is_char_boundary`), if `end` does not point
/// to the ending byte offset of a character (`end + 1` is either a starting
/// byte offset or equal to `len`), if `begin > end`, or if `end >= len`.
unsafe impl const SliceIndex<unicode_str> for ops::RangeInclusive<usize> {
    type Output = unicode_str;

    #[inline]
    fn get(self, slice: &unicode_str) -> Option<&Self::Output> {
        if *self.end() == usize::MAX {
            None
        } else {
            into_slice_range(self).get(slice)
        }
    }
    #[inline]
    fn get_mut(self, slice: &mut unicode_str) -> Option<&mut Self::Output> {
        if *self.end() == usize::MAX {
            None
        } else {
            into_slice_range(self).get_mut(slice)
        }
    }
    #[inline]
    unsafe fn get_unchecked(self, slice: *const unicode_str) -> *const Self::Output {
        // SAFETY: the caller must uphold the safety contract for `get_unchecked`.
        unsafe { into_slice_range(self).get_unchecked(slice) }
    }
    #[inline]
    unsafe fn get_unchecked_mut(self, slice: *mut unicode_str) -> *mut Self::Output {
        // SAFETY: the caller must uphold the safety contract for `get_unchecked_mut`.
        unsafe { into_slice_range(self).get_unchecked_mut(slice) }
    }
    #[inline]
    fn index(self, slice: &unicode_str) -> &Self::Output {
        if *self.end() == usize::MAX {
            str_index_overflow_fail();
        }
        into_slice_range(self).index(slice)
    }
    #[inline]
    fn index_mut(self, slice: &mut unicode_str) -> &mut Self::Output {
        if *self.end() == usize::MAX {
            str_index_overflow_fail();
        }
        into_slice_range(self).index_mut(slice)
    }
}

/// Implements substring slicing with syntax `&self[..= end]` or `&mut
/// self[..= end]`.
///
/// Returns a slice of the given string from the byte range \[0, `end`\].
/// Equivalent to `&self [0 .. end + 1]`, except if `end` has the maximum
/// value for `usize`.
///
/// This operation is *O*(1).
///
/// # Panics
///
/// Panics if `end` does not point to the ending byte offset of a character
/// (`end + 1` is either a starting byte offset as defined by
/// `is_char_boundary`, or equal to `len`), or if `end >= len`.
unsafe impl const SliceIndex<unicode_str> for ops::RangeToInclusive<usize> {
    type Output = unicode_str;

    #[inline]
    fn get(self, slice: &unicode_str) -> Option<&Self::Output> {
        if self.end == usize::MAX {
            None
        } else {
            (..self.end + 1).get(slice)
        }
    }
    #[inline]
    fn get_mut(self, slice: &mut unicode_str) -> Option<&mut Self::Output> {
        if self.end == usize::MAX {
            None
        } else {
            (..self.end + 1).get_mut(slice)
        }
    }
    #[inline]
    unsafe fn get_unchecked(self, slice: *const unicode_str) -> *const Self::Output {
        // SAFETY: the caller must uphold the safety contract for `get_unchecked`.
        unsafe { (..self.end + 1).get_unchecked(slice) }
    }
    #[inline]
    unsafe fn get_unchecked_mut(self, slice: *mut unicode_str) -> *mut Self::Output {
        // SAFETY: the caller must uphold the safety contract for `get_unchecked_mut`.
        unsafe { (..self.end + 1).get_unchecked_mut(slice) }
    }
    #[inline]
    fn index(self, slice: &unicode_str) -> &Self::Output {
        if self.end == usize::MAX {
            str_index_overflow_fail();
        }
        (..self.end + 1).index(slice)
    }
    #[inline]
    fn index_mut(self, slice: &mut unicode_str) -> &mut Self::Output {
        if self.end == usize::MAX {
            str_index_overflow_fail();
        }
        (..self.end + 1).index_mut(slice)
    }
}

struct LocalRangeInclusive<Idx> {
    // Note that the fields here are not public to allow changing the
    // representation in the future; in particular, while we could plausibly
    // expose start/end, modifying them without changing (future/current)
    // private fields may lead to incorrect behavior, so we don't want to
    // support that mode.
    pub(crate) start: Idx,
    pub(crate) end: Idx,

    // This field is:
    //  - `false` upon construction
    //  - `false` when iteration has yielded an element and the iterator is not exhausted
    //  - `true` when iteration has been used to exhaust the iterator
    //
    // This is required to support PartialEq and Hash without a PartialOrd bound or specialization.
    pub(crate) exhausted: bool,
}

// Copied from core::src::ops::range because the function was made private to that crate...
#[inline]
pub(crate) const fn into_slice_range(range: ops::RangeInclusive<usize>) -> ops::Range<usize> {
    let range: LocalRangeInclusive<usize> = unsafe { std::mem::transmute(range) };
    // If we're not exhausted, we want to simply slice `start..end + 1`.
    // If we are exhausted, then slicing with `end + 1..end + 1` gives us an
    // empty range that is still subject to bounds-checks for that endpoint.
    let exclusive_end = range.end + 1;
    let start = if range.exhausted {
        exclusive_end
    } else {
        range.start
    };
    start..exclusive_end
}


#[inline(never)]
#[cold]
#[track_caller]
const fn slice_error_fail() -> ! {
    panic!("failed to slice string");
}
