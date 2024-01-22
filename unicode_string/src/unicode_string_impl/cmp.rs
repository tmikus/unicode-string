use crate::{unicode_str, UnicodeString};
use std::borrow::Cow;

impl PartialEq for UnicodeString {
    #[inline]
    fn eq(&self, other: &UnicodeString) -> bool {
        PartialEq::eq(&self[..], &other[..])
    }
    #[inline]
    fn ne(&self, other: &UnicodeString) -> bool {
        PartialEq::ne(&self[..], &other[..])
    }
}

macro_rules! impl_eq {
    ($lhs:ty, $rhs: ty) => {
        #[allow(unused_lifetimes)]
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                PartialEq::eq(&self[..], &other[..])
            }
            #[inline]
            fn ne(&self, other: &$rhs) -> bool {
                PartialEq::ne(&self[..], &other[..])
            }
        }

        #[allow(unused_lifetimes)]
        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                PartialEq::eq(&self[..], &other[..])
            }
            #[inline]
            fn ne(&self, other: &$lhs) -> bool {
                PartialEq::ne(&self[..], &other[..])
            }
        }
    };
}

impl_eq! { UnicodeString, unicode_str }
impl_eq! { UnicodeString, &'a unicode_str }
#[cfg(not(no_global_oom_handling))]
impl_eq! { Cow<'a, unicode_str>, unicode_str }
#[cfg(not(no_global_oom_handling))]
impl_eq! { Cow<'a, unicode_str>, &'b unicode_str }
#[cfg(not(no_global_oom_handling))]
impl_eq! { Cow<'a, unicode_str>, UnicodeString }

