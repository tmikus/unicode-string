use std::mem;
use crate::UnicodeString;

#[repr(C)]
pub struct unicode_str {
    pub(crate) chars: [char],
}

impl unicode_str {
    /// Returns a slice of characters from this string slice.
    ///
    /// It is important to remember that [`char`] represents a Unicode Scalar Value, and might not match your
    /// idea of what a 'character' is. Iteration ver grapheme clusters may be what you actually want. This functionality
    /// is not provided by this library, check crates.io instead.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use unicode_string::ustr;
    ///
    /// let word = ustr!("goodbye");
    ///
    /// let count = word.chars().len();
    /// assert_eq!(7, count);
    ///
    /// let chars = word.chars();
    /// assert_eq!('g', chars[0]);
    /// assert_eq!('o', chars[1]);
    /// assert_eq!('o', chars[2]);
    /// assert_eq!('d', chars[3]);
    /// assert_eq!('b', chars[4]);
    /// assert_eq!('y', chars[5]);
    /// assert_eq!('e', chars[6]);
    /// ```
    ///
    /// Remember, [`char`]s might not match your intuition about characters:
    ///
    /// ```
    /// use unicode_string::ustr;
    ///
    /// let y = ustr!("y̆");
    ///
    /// let chars = y.chars();
    /// assert_eq!('y', chars[0]);
    /// assert_eq!('\u{0306}', chars[1]);
    /// ```
    #[inline]
    pub const fn chars(&self) -> &[char] {
        &self.chars
    }

    /// Returns the length of a unicode_str
    ///
    /// The returned value is the number of **characters**, not the number of bytes.
    ///
    #[inline]
    pub const fn len(&self) -> usize {
        self.chars.len()
    }

    /// Converts a slice of chars to a string slice.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use unicode_string::{unicode_str, ustr};
    ///
    /// // some bytes, in a vector
    /// let sparkle_heart = vec!['💖'];
    ///
    /// let sparkle_heart = unicode_str::from_chars(&sparkle_heart);
    ///
    /// assert_eq!(ustr!("💖"), sparkle_heart);
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_chars(v: &[char]) -> &unicode_str {
        // SAFETY: the caller must guarantee that the bytes `v` are valid UTF-8.
        // Also relies on `&u32str` and `&[char]` having the same layout.
        unsafe { mem::transmute(v) }
    }

    /// Converts a slice of bytes to a string slice without checking
    /// that the string contains valid UTF-8; mutable version.
    ///
    /// See the immutable version, [`from_utf8_unchecked()`] for more information.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use unicode_string::{unicode_str, ustr};
    ///
    /// let mut heart = vec!['💖'];
    /// let heart = unicode_str::from_chars_mut(&mut heart);
    ///
    /// assert_eq!(ustr!("💖"), heart);
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_chars_mut(v: &mut [char]) -> &mut unicode_str {
        // SAFETY: the caller must guarantee that the bytes `v`
        // are valid UTF-8, thus the cast to `*mut str` is safe.
        // Also, the pointer dereference is safe because that pointer
        // comes from a reference which is guaranteed to be valid for writes.
        unsafe { &mut *(v as *mut [char] as *mut unicode_str) }
    }
}

impl AsRef<[char]> for unicode_str {
    #[inline]
    fn as_ref(&self) -> &[char] {
        self.chars()
    }
}

impl const Default for &unicode_str {
    /// Creates an empty str
    #[inline]
    fn default() -> Self {
        unsafe { &*(&[] as *const [char] as *const unicode_str) }
    }
}

impl ToOwned for unicode_str {
    type Owned = UnicodeString;

    #[inline]
    fn to_owned(&self) -> Self::Owned {
        Self::Owned {
            vec: self.chars().to_vec(),
        }
    }

    fn clone_into(&self, target: &mut UnicodeString) {
        target.vec = self.chars().to_vec();
    }
}

impl std::fmt::Debug for unicode_str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", &String::from_iter(self.chars().iter()))
    }
}

impl std::fmt::Display for unicode_str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.pad(&String::from_iter(self.chars().iter()))
    }
}
