use core::ops::{
    Range, 
    RangeInclusive, 
    RangeFrom, 
    RangeTo, 
    RangeToInclusive, 
    RangeFull
};

use crate::StringIterable;

/// A `usize` or a range representing a slice of chars in a string.
/// 
/// This is similar to [`SliceIndex`](core::slice::SliceIndex).
pub trait StringIndex {
    fn start(&self) -> usize;
    fn len(&self) -> Option<usize>;
}

impl StringIndex for usize {
    fn start(&self) -> usize {
        *self
    }

    fn len(&self) -> Option<usize> {
        Some(1)
    }
}

impl StringIndex for Range<usize> {
    fn start(&self) -> usize {
        self.start
    }

    fn len(&self) -> Option<usize> {
        Some(self.end - self.start)
    }
}

impl StringIndex for RangeInclusive<usize> {
    fn start(&self) -> usize {
        *self.start()
    }

    fn len(&self) -> Option<usize> {
        Some(self.end() - self.start() + 1)
    }
}

impl StringIndex for RangeFrom<usize> {
    fn start(&self) -> usize {
        self.start
    }

    fn len(&self) -> Option<usize> {
        None
    }
}

impl StringIndex for RangeTo<usize> {
    fn start(&self) -> usize {
        0
    }

    fn len(&self) -> Option<usize> {
        Some(self.end)
    }
}

impl StringIndex for RangeToInclusive<usize> {
    fn start(&self) -> usize {
        0
    }

    fn len(&self) -> Option<usize> {
        Some(self.end + 1)
    }
}

impl StringIndex for RangeFull {
    fn start(&self) -> usize {
        0
    }

    fn len(&self) -> Option<usize> {
        None
    }
}

/// Extension methods for strings
pub trait StringExt {

    /// Try obtain a substring with a given index or range.
    /// 
    /// Returns `Ok(&str)` if the exact substring is found.
    /// 
    /// Returns `Err(&str)` if [`len`](StringIndex::len) is bound and less than `len` [`char`]s found.
    fn try_substr<'t>(&'t self, idx: impl StringIndex) -> Result<&'t str, &'t str>;

    /// Obtain a substring with a given index or range.
    fn substr<'t>(&'t self, idx: impl StringIndex) -> &'t str{
        match self.try_substr(idx) {
            Ok(s) => s,
            Err(s) => s,
        }
    }

    /// Try obtain a substring with a given index or range in reverse.
    /// 
    /// Returns `Ok(&str)` if the exact substring is found.
    /// 
    /// Returns `Err(&str)` if [`len`](StringIndex::len) is bound and less than `len` [`char`]s found.
    fn try_substr_back<'t>(&'t self, idx: impl StringIndex) -> Result<&'t str, &'t str>;
    
    /// Obtain a substring with a given index or range in reverse.
    fn substr_back<'t>(&'t self, idx: impl StringIndex) -> &'t str{
        match self.try_substr_back(idx) {
            Ok(s) => s,
            Err(s) => s,
        }
    }

    /// Concatenate adjacent substrings.
    /// 
    /// Returns `None` if `first` and `second` are not adjacent or 
    /// `first` and `second` are not substrings of `self`.
    /// 
    /// # Examples
    /// 
    /// Correst usage:
    /// ```
    /// # use string_iter::StringExt;
    /// let parent = "foobar";
    /// let foo = &parent[0..3];
    /// let bar = &parent[3..6];
    /// assert_eq!(parent.merge(foo, bar), Some("foobar"));
    /// 
    /// // has to be in the correct order
    /// assert_eq!(parent.merge(bar, foo), None);
    /// ```
    /// 
    /// Not substrings: 
    /// ```
    /// # use string_iter::StringExt;
    /// let parent = "foobar";
    /// let foo = "foo";
    /// let bar = "bar";
    /// assert_eq!(parent.merge(foo, bar), None);
    /// ```
    /// 
    /// Not adjacent: 
    /// ```
    /// # use string_iter::StringExt;
    /// let parent = "foobar";
    /// let fo = &parent[0..2];
    /// let bar = &parent[3..6];
    /// assert_eq!(parent.merge(fo, bar), None);
    /// ```
    /// 
    /// Overlapping: 
    /// ```
    /// # use string_iter::StringExt;
    /// let parent = "foobar";
    /// let foob = &parent[0..4];
    /// let obar = &parent[2..6];
    /// assert_eq!(parent.merge(foob, obar), None);
    /// ```
    fn merge<'t>(&'t self, first: &str, second: &str) -> Option<&'t str>;
}

impl<T> StringExt for T where T: AsRef<str> {
    
    fn try_substr<'t>(&'t self, idx: impl StringIndex) -> Result<&'t str, &'t str> {
        let mut iter = self.str_iter();
        iter.skip_front(idx.start());
        match idx.len() {
            Some(n) => iter.peekn(n),
            None => Ok(iter.as_str()),
        }
    }

    fn try_substr_back<'t>(&'t self, idx: impl StringIndex) -> Result<&'t str, &'t str> {
        let mut iter = self.str_iter();
        iter.skip_back(idx.start());
        match idx.len() {
            Some(n) => iter.peekn_back(n),
            None => Ok(iter.as_str()),
        }
    }

    fn merge<'t>(&'t self, first: &str, second: &str) -> Option<&'t str> {
        crate::merge::merge(self.as_ref(), first, second)
    }
}
