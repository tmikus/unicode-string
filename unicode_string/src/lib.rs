#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(slice_index_methods)]

mod unicode_str_impl;
mod unicode_string_impl;

pub use self::unicode_str_impl::*;
pub use self::unicode_string_impl::*;
pub use unicode_string_macros::*;
