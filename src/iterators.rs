
use core::ops::{Deref, DerefMut};
use core::iter::FusedIterator;
use crate::StringIter;


impl<'t> Iterator for StringIter<'t> {
    type Item = (char, &'t str);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.next_char()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.str.len(), Some(self.str.len()))
    }

    fn count(self) -> usize {
        self.str.chars().count()
    }
}

impl<'t> DoubleEndedIterator for StringIter<'t> {
    
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next_char_back()
    }
}

impl<'t> FusedIterator for StringIter<'t> {}

impl<'t> StringIter<'t> {
    
    /// Map the iterator into an `Iterator<Item = char>`.
    /// 
    /// Discarding the str output
    pub fn chars(self) -> CharIter<'t>{
        CharIter(self)
    }

    /// Map the iterator into an `Iterator<Item = &str>`.
    /// 
    /// Discarding the char output
    pub fn strs(self) -> StrIter<'t>{
        StrIter(self)
    }

    /// Map the iterator into an `Iterator<Item = u8>`.
    /// 
    /// where u8 is the first byte of the &str.
    pub fn ascii(self) -> AsciiIter<'t> {
        AsciiIter(self)
    }

    /// Map the iterator into an `Iterator<Item = (u8, &str)>`.
    /// 
    /// where u8 is the first byte of the &str.
    pub fn ascii_str(self) -> AsciiStrIter<'t> {
        AsciiStrIter(self)
    }

    /// Make the iterator peek for `len`.
    pub fn look_ahead(self, len: usize) -> LookAhead<'t> {
        assert!(len != 0, "look_ahead cannot be 0");
        LookAhead { iter: self, look_ahead: len }
    }
}

macro_rules! alt_iter {
    ($name: ident, $base:ident, $item: ty, $func: expr, $doc: literal) => {

        #[doc = $doc]
        #[repr(transparent)]
        #[derive(Debug, Clone)]
        pub struct $name<'t>($base<'t>);

        impl<'t> Deref for $name<'t> {
            type Target = $base<'t>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'t> DerefMut for $name<'t> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl core::borrow::Borrow<str> for $name<'_> {
            fn borrow(&self) -> &str {
                self.0.as_str()
            }
        }

        impl AsRef<str> for $name<'_> {
            fn as_ref(&self) -> &str {
                self.0.as_str()
            }
        }

        impl<'t> Iterator for $name<'t> {
            type Item = $item;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.0.next().map($func)
            }


            fn size_hint(&self) -> (usize, Option<usize>) {
                self.0.size_hint()
            }

            fn count(self) -> usize {
                self.0.count()
            }
        }

        impl<'t> DoubleEndedIterator for $name<'t> {
            #[inline]
            fn next_back(&mut self) -> Option<Self::Item> {
                self.0.next_back().map($func)
            }
        }

        impl<'t> FusedIterator for $name<'t> {}
    };
}

alt_iter!(CharIter, StringIter, char, |(c, _)| c,
    "A mapped [`StringIter`] that yields [`char`]s.");
alt_iter!(StrIter, StringIter, &'t str, |(_, s)| s,
    "A mapped [`StringIter`] that yields [`&str`]s.");
alt_iter!(AsciiIter, StringIter, u8, |(_, s)| unsafe {*s.as_bytes().get_unchecked(0)},
    "A mapped [`StringIter`] that yields [`u8`]s.");
alt_iter!(AsciiStrIter, StringIter, (u8, &'t str), |(_, s)| (unsafe {*s.as_bytes().get_unchecked(0)}, s),
    "A mapped [`StringIter`] that yields `(u8, &str)`s.");


/// A mapped StringIter that yields longer [`&str`]s by looking ahead.
#[derive(Debug, Clone)]
pub struct LookAhead<'t>{
    iter: StringIter<'t>,
    look_ahead: usize,
}

impl<'t> Deref for LookAhead<'t> {
    type Target = StringIter<'t>;

    fn deref(&self) -> &Self::Target {
        &self.iter
    }
}

impl<'t> DerefMut for LookAhead<'t> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.iter
    }
}

impl<'t> Iterator for LookAhead<'t> {
    type Item = (char, &'t str);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let s = match self.iter.peekn(self.look_ahead) {
            Ok(s) => s,
            Err(s) => s,
        };
        self.iter.next().map(|(c, _)| (c, s))
    }


    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize {
        self.iter.count()
    }
}

impl<'t> DoubleEndedIterator for LookAhead<'t> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let s = match self.iter.peekn_back(self.look_ahead) {
            Ok(s) => s,
            Err(s) => s,
        };
        self.iter.next().map(|(c, _)| (c, s))
    }
}

impl<'t> FusedIterator for LookAhead<'t> {}

impl<'t> LookAhead<'t> {

    /// Map the iterator into an `Iterator<Item = &str>`.
    pub fn strs(self) -> LookAheadStrIter<'t>{
        LookAheadStrIter(self)
    }
}

alt_iter!(LookAheadStrIter, LookAhead, &'t str, |(_, s)| s,
    "A mapped [`LookAhead`] that yields [`&str`]s.");
