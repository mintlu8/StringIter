#![no_std]
//! An overly designed [`&str`] iterator made
//! with zero-copy parsing in mind, with an emphasis on ergonomics.
//! 
//! # Usage
//! 
//! `StringIter` offers iteration and pattern matching methods 
//! as well as methods normally found in string types 
//! that would make sense for an iterator.
//! 
//! The standard StringIter yields a char in both its [`char`] and [`&str`]
//! representations, allowing easily storage in its [`&str`] or [`Cow<str>`](std::borrow::Cow) form.
//! 
//! * Trimming
//! 
//! ```
//! # use string_iter::prelude::*;
//! let mut iter = "  !#$@!foo&*  ".str_iter();
//! iter.trim();
//! assert_eq!(iter.as_str(), "!#$@!foo&*");
//! iter.trim_start_by(|x: char| !x.is_alphabetic());
//! assert_eq!(iter.as_str(), "foo&*");
//! iter.trim_end_by(|x: char| !x.is_alphabetic());
//! assert_eq!(iter.as_str(), "foo");
//! ```
//! 
//! * Peeking
//! 
//! ```
//! # use string_iter::prelude::*;
//! let mut iter = "bar".str_iter();
//! assert_eq!(iter.peek(), Some(('b', "b")));
//! assert_eq!(iter.peek_back(), Some(('r', "r")));
//! assert_eq!(iter.peekn(2), Ok("ba"));
//! assert_eq!(iter.peekn_back(2), Ok("ar"));
//! assert_eq!(iter.peekn(4), Err("bar"));
//! assert_eq!(iter.peekn_back(4), Err("bar"));
//! ```
//! * Iterating
//! 
//! ```
//! # use string_iter::prelude::*;
//! let chars = [('😀', "😀"), ('🙁', "🙁"), ('😡', "😡"), ('😱', "😱")];
//! for (a, b) in "😀🙁😡😱".str_iter().zip(chars.into_iter()) {
//!     assert_eq!(a, b);
//! }
//! ```
//! 
//! * Look-ahead
//! 
//! ```
//! # use string_iter::prelude::*;
//! let mut iter = "蟹🦀a🚀𓄇ë".str_iter().look_ahead(2).strs();
//! assert_eq!(iter.next(), Some("蟹🦀"));
//! assert_eq!(iter.next(), Some("🦀a"));
//! assert_eq!(iter.next(), Some("a🚀"));
//! assert_eq!(iter.next(), Some("🚀𓄇"));
//! assert_eq!(iter.next(), Some("𓄇ë"));
//! assert_eq!(iter.next(), Some("ë"));
//! assert_eq!(iter.next(), None);
//! ```
//! 
//! * Slice by pattern
//! ```
//! # use string_iter::prelude::*;
//! let mut iter = "{{foo}bar}baz".str_iter();
//! let mut count = 0;
//! let s = iter.next_slice((|x| {
//!     match x {
//!         '{' => count += 1,
//!         '}' => count -= 1,
//!         _ => (),
//!     };
//!     count == 0
//! }).sep_with(Sep::Yield));
//! assert_eq!(s, Some("{{foo}bar}"));
//! assert_eq!(iter.as_str(), "baz");
//! ```
//! 
//! * Splitting
//! 
//! ```
//! # use string_iter::prelude::*;
//! let mut iter = "thisIsCamelCase"
//!     .str_iter()
//!     .into_substrs(|c: char| c.is_uppercase());
//! assert_eq!(iter.next(), Some("this"));
//! assert_eq!(iter.next(), Some("Is"));
//! assert_eq!(iter.next(), Some("Camel"));
//! assert_eq!(iter.next(), Some("Case"));
//! assert_eq!(iter.next(), None);
//! ```
//! 
//! # Patterns
//! 
//! We use [`Patterns`](Pattern) in [`trim`](StringIter::trim_by), 
//! [`slice`](StringIter::try_next_slice) and 
//! [`split`](StringIter::into_substrs).
//! 
//! In [`trim`](StringIter::trim_by), the pattern matches until a false value is found.
//! 
//! In [`slice`](StringIter::try_next_slice) and 
//! [`split`](StringIter::into_substrs), the pattern matches until a true value is found.
//! 
//! See [`Sep`] and [`sep_with()`](SetSep) for dealing with the corner case.
//! 
//! ## Supported Patterns
//! 
//! * [`isize`]
//! 
//! Matches once on the nth `char`.
//! 
//! * `..isize`
//! 
//! Matches the first `n` `char`s. 
//! This is useful with [`trim`](StringIter::trim_by).
//! 
//! * [`char`]
//! 
//! Matches a char.
//! 
//! * [`&str`]
//! 
//! Matching an `&str` by looking ahead.
//! 
//! * `&[char]` or `[char;N]`
//! 
//! Matches any char in the set.
//! 
//! * `char..=char`
//! 
//! Matches a char in range, 
//! we only support inclusive ranges to avoid errors.
//! 
//! * `FnMut(char) -> FallibleBool`
//! 
//! Matches any char that makes the function return true.
//! 
//! [`FallibleBool`] can be [`bool`], [`Option<bool>`] or [`Result<bool, E: Debug>`]
//! 
//! * `(FnMut(&str) -> FallibleBool).expecting(n)`
//! 
//! Matches any [`&str`] that makes the function return true
//! by looking ahead for `n` `char`s.
//! 
//! * `(FnMut(char, &str) -> FallibleBool).expecting(n)`
//! 
//! Matches any [`&str`] that makes the function return true
//! by looking ahead for `n` `char`s.
//! 
//! `char` is the first [`char`] in [`&str`]
//! 
//! * [`Interval`](patterns::Interval) or [`interval!()`](`interval!`)
//! 
//! Match repeatedly by an interval.
//! 
//! * [`pat!()`](pat!)
//! 
//! A macro that turns `match` patterns into [`Pattern`]s.
//! 
//! * Custom implementations of [`Pattern`]
//! 
//! You can write your own pattern types!
//! 
//! # Examples
//! 
//! Getting an ascii identifier from a string
//! ```
//! # use string_iter::prelude::*;
//! let foo = r#"  ferris123@crab.io "#;
//! let mut iter = foo.str_iter();
//! iter.trim_start();
//! let mut quotes = 0;
//! let slice = match iter.peek() {
//!     Some(('a'..='z'|'A'..='Z'|'_', _)) => {
//!         iter.next_slice(pat!(!'a'..='z'|'A'..='Z'|'0'..='9'|'_'))
//!     }
//!     _ => panic!("expected ident")
//! };
//! assert_eq!(slice, Some("ferris123"));
//! 
//! // note @ is still in the iterator
//! assert_eq!(iter.as_str(), "@crab.io ");
//! ```
//! 
//! Getting a string literal "foo" from a string:
//! ```
//! # use string_iter::prelude::*;
//! let foo = r#"    "foo"  bar "#;
//! let mut iter = foo.str_iter();
//! iter.trim_start();
//! let mut quotes = 0;
//! let slice = iter.next_slice((|c| match c {
//!     '"' =>  {
//!         quotes += 1;
//!         quotes == 2
//!     }
//!     _ => false,
//! }).sep_with(Sep::Yield));
//! assert_eq!(slice, Some("\"foo\""));
//! assert_eq!(iter.as_str(), "  bar ");
//! ```
//! 
//! # Performance
//! 
//! This crate is comparable in speed to [`str::chars()`].
//! 
//! If operating on [`char`]s alone, [`str::chars()`] is faster.
//! 
//! But [`StringIter`] can be faster than [`str::chars()`]
//! if you need to convert the [`char`] back into UTF-8.
//! 
//! # Safety
//! 
//! This crate uses **a lot** of unsafe code to take advantage of the
//! UTF-8 invarient and bypass some bounds checks and UTF-8 checks.
//! 
//! In addition we do not guarantee memory safety if given invalid UTF-8 input.
//! 
//! Please file an issue if you find any soundness problem.

use core::{borrow::Borrow, fmt::Display};
mod slice;
mod merge;
mod split;
mod iter_fns;
mod interval;
mod pattern;
mod iterators;
mod string_ext;

pub use merge::Merge;
pub use string_ext::{StringExt, StringIndex};

pub use pattern::{
    Pattern,
    Sep, 
    SetSep,
    Never,
    FallibleBool,
    CharStrPredicate, StrPredicate
};

pub mod iter {
    //! Misallenious iterators used in this crate.
    //! 
    //! Mapped iterators share regular methods with [`StringIter`](crate::StringIter)
    //! and are functionally identical.
    pub use crate::iterators::*;
    pub use crate::merge::MergeIter;
    pub use crate::split::SplitIter;
}
pub mod patterns {
    //! Misallenious patterns used in this crate.
    pub use crate::pattern:: {
        SizedCharStrPredicate, 
        SizedStrPredicate,
        SepConfig,
    };
    pub use crate::interval::Interval;
}


pub mod prelude {
    //! Convenience re-export of common members
    //! ```
    //! use string_iter::prelude::*;
    //! ```
    #[doc(no_inline)]
    pub use crate::StringIterable;
    #[doc(no_inline)]
    pub use crate::string_ext::StringExt;
    #[doc(no_inline)]
    pub use crate::pattern::{Sep, SetSep, CharStrPredicate, StrPredicate};
    #[doc(no_inline)]
    pub use crate::merge::Merge;
    pub use crate::interval;
    pub use crate::pat;
}

/// A struct that can be iterated with a [`StringIter`]
pub trait StringIterable {
    /// Construct a new [`StringIter`]
    fn str_iter<'t>(&'t self) -> StringIter<'t>;
}

impl<T> StringIterable for T where T: AsRef<str>{
    fn str_iter<'t>(&'t self) -> StringIter<'t> {
        StringIter { str: self.as_ref() }
    }
}

/// A double ended, UTF-8 [`char`] based [`Iterator`] for [`&str`]s that 
/// supports iterating, looking ahead, trimming, pattern matching, splitting
/// and other common string operations.
/// 
/// Also a drop in replacement for an [`AsRef<str>`] or a [`Borrow<str>`].
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct StringIter<'t>{
    str: &'t str,
}

impl<'t> StringIter<'t> {
    /// Construct a new StringIter from a &str
    pub const fn new(s: &'t str) -> Self {
        StringIter {
            str: s
        }
    }

    /// Returns the length of the underlying [`str`] in bytes.
    pub const fn len(&self) -> usize{
        self.str.len()
    }

    /// Returns `true` if the underlying [`str`] has a length of zero bytes.
    pub const fn is_empty(&self) -> bool{
        self.str.is_empty()
    }

    /// Returns the underlying [`str`] of this [`StringIter`]
    pub const fn as_str(&self) -> &'t str {
        self.str
    }

    /// Returns the underlying `[u8]` of this [`StringIter`]
    pub const fn as_bytes(&self) -> &'t[u8] {
        self.str.as_bytes()
    }

    unsafe fn slice_front_ptr(&self, ptr: *const u8) -> &'t str{
        let len = ptr as usize - self.str.as_ptr() as usize;
        self.str.get_unchecked(..len)
    }

    unsafe fn slice_back_ptr(&self, ptr: *const u8) -> &'t str{
        let len = ptr as usize - self.str.as_ptr() as usize;
        self.str.get_unchecked(len..)
    }

    /// Returns true if the given [`&str`] matches the prefix of the underlying [`str`]
    ///
    /// Returns false if it does not.
    pub fn startswith(&self, s: &str) -> bool{
        self.str.starts_with(s)
    }

    /// Returns true if the given [`&str`] matches the suffix of the underlying [`str`]
    ///
    /// Returns false if it does not.
    pub fn endswith(&self, s: &str) -> bool{
        self.str.ends_with(s)
    }

    /// Removes leading and trailing whitespaces from this [`StringIter`]
    pub fn trim(&mut self){
        self.str = self.str.trim()
    }

    /// Removes leading whitespaces from this [`StringIter`]
    pub fn trim_start(&mut self){
        self.str = self.str.trim_start()
    }

    /// Removes trailing whitespaces from this [`StringIter`]
    pub fn trim_end(&mut self){
        self.str = self.str.trim_end()
    }
    
    /// Skip `n` leading [`char`]s from this [`StringIter`], 
    /// returns `true` if the string is empty afterwards.
    pub fn skip_front(&mut self, n: usize) -> bool{
        for _ in 0..n{
            self.next();
        }
        self.is_empty()
    }

    /// Skip `n` trailing [`char`]s from this [`StringIter`], 
    /// returns `true` if the string is empty afterwards.
    pub fn skip_back(&mut self, n: usize) -> bool{
        for _ in 0..n{
            self.next_back();
        }
        self.is_empty()
    }
}

impl AsRef<str> for StringIter<'_> {
    fn as_ref(&self) -> &str {
        self.str
    }
}

impl Borrow<str> for StringIter<'_> {
    fn borrow(&self) -> &str {
        self.str
    }
}

impl<'t> From<&'t str> for StringIter<'t> {
    fn from(value: &'t str) -> Self {
        Self { str: value }
    }
}

impl<'t> Into<&'t str> for StringIter<'t> {
    fn into(self) -> &'t str {
        self.str
    }
}

impl<'t> Display for StringIter<'t> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.str)
    }
}

impl PartialEq<str> for StringIter<'_> {
    fn eq(&self, other: &str) -> bool {
        self.str == other
    }
}

impl PartialOrd<str> for StringIter<'_> {
    fn partial_cmp(&self, other: &str) -> Option<core::cmp::Ordering> {
        self.str.partial_cmp(other)
    }
}


impl PartialEq<&str> for StringIter<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.str == *other
    }
}

impl PartialOrd<&str> for StringIter<'_> {
    fn partial_cmp(&self, other: &&str) -> Option<core::cmp::Ordering> {
        self.str.partial_cmp(other)
    }
}

#[cfg(feature="std")]
const _: () = {
    extern crate alloc;
    use alloc::boxed::Box;
    use alloc::rc::Rc;
    use alloc::string::String;
    use alloc::borrow::Cow;
    use alloc::sync::Arc;

    impl<'t> Into<String> for StringIter<'t> {
        fn into(self) -> String {
            self.str.into()
        }
    }

    impl<'t> Into<Box<str>> for StringIter<'t> {
        fn into(self) -> Box<str> {
            self.str.into()
        }
    }

    impl<'t> Into<Rc<str>> for StringIter<'t> {
        fn into(self) -> Rc<str> {
            self.str.into()
        }
    }

    impl<'t> Into<Arc<str>> for StringIter<'t> {
        fn into(self) -> Arc<str> {
            self.str.into()
        }
    }

    impl<'t> Into<Cow<'t, str>> for StringIter<'t> {
        fn into(self) -> Cow<'t, str> {
            Cow::Borrowed(self.str)
        }
    }

    impl<'t> From<&'t String> for StringIter<'t> {
        fn from(s: &'t String) -> Self {
            Self::new(s.as_ref())
        }
    }

    impl<'t> From<&'t Box<str>> for StringIter<'t> {
        fn from(s: &'t Box<str>) -> Self {
            Self::new(s.as_ref())
        }
    }

    impl<'t> From<&'t Rc<str>> for StringIter<'t> {
        fn from(s: &'t Rc<str>) -> Self {
            Self::new(s.as_ref())
        }
    }

    impl<'t> From<&'t Arc<str>> for StringIter<'t> {
        fn from(s: &'t Arc<str>) -> Self {
            Self::new(s.as_ref())
        }
    }

    impl<'a, 't: 'a> From<&'t Cow<'a, str>> for StringIter<'t> {
        fn from(s: &'t Cow<'a, str>) -> Self {
            Self::new(s.as_ref())
        }
    }

    /// This conversion only works if the [`Cow`] is Borrowed
    impl<'t> TryFrom<Cow<'t, str>> for StringIter<'t> {
        type Error = ();
        fn try_from(cow: Cow<'t, str>) -> Result<Self, ()> {
            match cow {
                Cow::Borrowed(s) => Ok(Self { str: s }),
                Cow::Owned(_) => Err(()),
            }
        }
    }
};
