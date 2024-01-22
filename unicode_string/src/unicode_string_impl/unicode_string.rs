use std::borrow::{Borrow, BorrowMut};
use std::{ops, str};
use crate::{FromUtf8Error, unicode_str};

#[derive(Debug, PartialOrd, Eq, Ord)]
pub struct UnicodeString {
    pub(crate) vec: Vec<char>,
}

impl UnicodeString {
    /// Creates a new empty `UnicodeString`.
    ///
    /// Given that the `UnicodeString` is empty, this will not allocate any initial
    /// buffer. While that means that this initial operation is very
    /// inexpensive, it may cause excessive allocation later when you add
    /// data. If you have an idea of how much data the `UnicodeString` will hold,
    /// consider the [`with_capacity`] method to prevent excessive
    /// re-allocation.
    ///
    /// [`with_capacity`]: UnicodeString::with_capacity
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use unicode_string::UnicodeString;
    ///
    /// let s = UnicodeString::new();
    /// ```
    pub const fn new() -> UnicodeString {
        UnicodeString { vec: Vec::new() }
    }

    /// Creates a new empty `UnicodeString` with a particular capacity.
    ///
    /// `UnicodeString`s have an internal buffer to hold their data. The capacity is
    /// the length of that buffer, and can be queried with the [`capacity`]
    /// method. This method creates an empty `UnicodeString`, but one with an initial
    /// buffer that can hold `capacity` bytes. This is useful when you may be
    /// appending a bunch of data to the `UnicodeString`, reducing the number of
    /// reallocations it needs to do.
    ///
    /// [`capacity`]: UnicodeString::capacity
    ///
    /// If the given capacity is `0`, no allocation will occur, and this method
    /// is identical to the [`new`] method.
    ///
    /// [`new`]: UnicodeString::new
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use unicode_string::UnicodeString;
    ///
    /// let mut s = UnicodeString::with_capacity(10);
    ///
    /// // The UnicodeString contains no chars, even though it has capacity for more
    /// assert_eq!(s.len(), 0);
    ///
    /// // These are all done without reallocating...
    /// let cap = s.capacity();
    /// for _ in 0..10 {
    ///     s.push('a');
    /// }
    ///
    /// assert_eq!(s.capacity(), cap);
    ///
    /// // ...but this may make the string reallocate
    /// s.push('a');
    /// ```
    pub fn with_capacity(capacity: usize) -> UnicodeString {
        UnicodeString { vec: Vec::with_capacity(capacity) }
    }

    /// Converts a [`&str`] to a `UnicodeString`.
    ///
    /// This method will make a new copy of the underlying string.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use unicode_string::{UnicodeString, ustr};
    ///
    /// assert_eq!(ustr!("Hello world"), UnicodeString::from_string("Hello world"));
    /// ```
    ///
    pub fn from_string(s: &str) -> Self {
        UnicodeString {
            vec: s.chars().collect(),
        }
    }

    /// Converts a vector of bytes to a `UnicodeString`.
    ///
    /// A string ([`UnicodeString`]) is made of bytes ([`u8`]), and a vector of bytes
    /// ([`Vec<u8>`]) is made of bytes, so this function converts between the
    /// two. Not all byte slices are valid `UnicodeString`s, however: `UnicodeString`
    /// requires that it is valid UTF-8. `from_utf8()` checks to ensure that
    /// the bytes are valid UTF-8, and then does the conversion.
    ///
    /// If you are sure that the byte slice is valid UTF-8, and you don't want
    /// to incur the overhead of the validity check, there is an unsafe version
    /// of this function, [`from_utf8_unchecked`], which has the same behavior
    /// but skips the check.
    ///
    /// This method will take care to not copy the vector, for efficiency's
    /// sake.
    ///
    /// If you need a [`&str`] instead of a `UnicodeString`, consider
    /// [`str::from_utf8`].
    ///
    /// The inverse of this method is [`into_bytes`].
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the slice is not UTF-8 with a description as to why the
    /// provided bytes are not UTF-8. The vector you moved in is also included.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use unicode_string::{UnicodeString, ustr};
    ///
    /// // some bytes, in a vector
    /// let sparkle_heart = vec![240, 159, 146, 150];
    ///
    /// // We know these bytes are valid, so we'll use `unwrap()`.
    /// let sparkle_heart = UnicodeString::from_utf8(sparkle_heart).unwrap();
    ///
    /// assert_eq!(ustr!("💖"), sparkle_heart);
    /// ```
    ///
    /// Incorrect bytes:
    ///
    /// ```
    /// use unicode_string::UnicodeString;
    ///
    /// // some invalid bytes, in a vector
    /// let sparkle_heart = vec![0, 159, 146, 150];
    ///
    /// assert!(UnicodeString::from_utf8(sparkle_heart).is_err());
    /// ```
    ///
    /// See the docs for [`FromUtf8Error`] for more details on what you can do
    /// with this error.
    ///
    /// [`from_utf8_unchecked`]: UnicodeString::from_utf8_unchecked
    /// [`Vec<u8>`]: crate::vec::Vec "Vec"
    /// [`&str`]: prim@str "&str"
    /// [`into_bytes`]: UnicodeString::into_bytes
    #[inline]
    pub fn from_utf8(vec: Vec<u8>) -> Result<UnicodeString, FromUtf8Error> {
        match str::from_utf8(&vec) {
            Ok(v) => Ok(UnicodeString {
                vec: v.chars().collect(),
            }),
            Err(e) => Err(FromUtf8Error {
                bytes: vec,
                error: e,
            }),
        }
    }

    /// Returns this `UnicodeString`'s capacity, in bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use unicode_string::UnicodeString;
    ///
    /// let s = UnicodeString::with_capacity(10);
    ///
    /// assert!(s.capacity() >= 10);
    /// ```
    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    /// Returns the length of this `UnicodeString`, in unicode chars. In other words, it might not be what a human considers the
    /// length of the string.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use unicode_string::UnicodeString;
    ///
    /// let a = UnicodeString::from_string("foo");
    /// assert_eq!(a.len(), 3);
    ///
    /// let fancy_f = UnicodeString::from_string("ƒoo");
    /// assert_eq!(fancy_f.len(), 3);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Appends the given [`char`] to the end of this `UnicodeString`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use unicode_string::{UnicodeString, ustr};
    ///
    /// let mut s = UnicodeString::from_string("abc");
    ///
    /// s.push('1');
    /// s.push('2');
    /// s.push('3');
    ///
    /// assert_eq!(ustr!("abc123"), s);
    /// ```
    #[cfg(not(no_global_oom_handling))]
    #[inline]
    pub fn push(&mut self, ch: char) {
        self.vec.push(ch)
    }
}

impl Borrow<unicode_str> for UnicodeString {
    #[inline]
    fn borrow(&self) -> &unicode_str {
        &self[..]
    }
}

impl BorrowMut<unicode_str> for UnicodeString {
    #[inline]
    fn borrow_mut(&mut self) -> &mut unicode_str {
        &mut self[..]
    }
}

impl Clone for UnicodeString {
    fn clone(&self) -> Self {
        UnicodeString {
            vec: self.vec.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.vec.clone_from(&source.vec);
    }
}


impl ops::Deref for UnicodeString {
    type Target = unicode_str;

    #[inline]
    fn deref(&self) -> &unicode_str {
        unicode_str::from_chars(&self.vec)
    }
}

impl ops::DerefMut for UnicodeString {
    #[inline]
    fn deref_mut(&mut self) -> &mut unicode_str {
        unicode_str::from_chars_mut(&mut *self.vec)
    }
}
