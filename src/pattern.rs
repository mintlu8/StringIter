use core::{fmt::Debug, num::NonZeroUsize};
use core::ops::{RangeTo, RangeInclusive};

/// A never type that cannot be instanciated.
#[derive(Debug)]
pub enum Never{}

/// This proves Pattern is object safe.
const _: Option<&dyn Pattern<Err = Never>> = None;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// Determines what to do with a matched [`char`] on string
/// separation. By default [`Retain`](Sep::Retain).
pub enum Sep{
    /// Keep the [`char`] in the iterator.
    Retain,
    /// Yield the [`char`] with the output.
    Yield,
    /// Discard the [`char`].
    Split,
    /// Yield the [`char`] but keep it in the iterator.
    Conjoin,
}

/// A pattern for use in 
/// [`slice`](crate::StringIter::try_next_slice), 
/// [`split`](crate::StringIter::into_substrs) and 
/// [`trim`](crate::StringIter::trim_by)
/// functions.
pub trait Pattern{
    type Err: Debug;
    /// Try matching a char in a pattern
    /// 
    /// # Arguments
    ///
    /// * `c` - The current [`char`], guaranteed to be the first [`char`] in `s`
    /// * `s` - The current [`&str`], a substring of some length after `c`
    /// 
    /// In [`slice`](StringIter::try_next_slice) or 
    /// [`split`](StringIter::into_substrs): match until the first `true` result
    /// 
    /// In [`trim_by`](StringIter::trim_by): match until the first `false` result
    /// 
    /// [`len()`](Pattern::len) is a suggestion for `s.len()`.
    /// However `s` is not guaranteed to have the same length as `self.len()`,
    /// since joining multiple patterns can increase `s.len()`, 
    /// and corner cases will decrease `s.len()`.
    fn matches(&mut self, c: char, s: &str) -> Result<bool, Self::Err>;
    /// Determines how many [`char`]s to look ahead, default `1`.
    /// 
    /// The iterator will not stop prematurely because of look-ahead.
    fn len(&self) -> NonZeroUsize { NonZeroUsize::new(1).unwrap() }
    /// Determines what to do with the matched [char] on separation.
    /// 
    /// See also [`sep_with`](SetSep::sep_with)
    fn sep(&self) -> Sep { Sep::Retain }
}

impl Pattern for isize {
    type Err = Never;
    /// similar to peekn
    fn matches(&mut self, _: char, _: &str) -> Result<bool, Self::Err> {
        *self -= 1;
        Ok(*self == -1)
    }
}

impl Pattern for RangeTo<isize> {
    type Err = Never;
    fn matches(&mut self, _: char, _: &str) -> Result<bool, Self::Err> {
        self.end -= 1;
        Ok(self.contains(&-1))
    }
}

impl Pattern for RangeInclusive<char> {
    type Err = Never;
    fn matches(&mut self, c: char, _: &str) -> Result<bool, Self::Err> {
        Ok(self.contains(&c))
    }
}

impl Pattern for char {
    type Err = Never;
    fn matches(&mut self, c: char, _: &str) -> Result<bool, Self::Err> {
        Ok(c == *self)
    }
}

impl Pattern for &str {
    type Err = Never;
    fn matches(&mut self, _: char, s: &str) -> Result<bool, Self::Err> {
        Ok(s.starts_with(*self))
    }
    fn len(&self) -> NonZeroUsize {
        NonZeroUsize::new(self.chars().count())
            .expect("\"\" is not a valid pattern")
    }
}

impl Pattern for &[char] {
    type Err = Never;
    fn matches(&mut self, c: char, _: &str) -> Result<bool, Self::Err> {
        Ok(self.contains(&c))
    }
}

impl<const N: usize> Pattern for [char; N] {
    type Err = Never;
    fn matches(&mut self, c: char, _: &str) -> Result<bool, Self::Err> {
        Ok(self.contains(&c))
    }
}

mod private{
    pub trait Sealed {}
}

/// A generalized fallible boolean result.
pub trait FallibleBool: private::Sealed {
    type Err: Debug;
    fn get(self) -> Result<bool, Self::Err>;
}

impl private::Sealed for bool {}

impl FallibleBool for bool {
    type Err = Never;

    fn get(self) -> Result<bool, Self::Err> {
        Ok(self)
    }
}

impl private::Sealed for Option<bool> {}

impl FallibleBool for Option<bool> {
    type Err = ();

    fn get(self) -> Result<bool, ()> {
        self.ok_or(())
    }
}

impl<E: Debug> private::Sealed for Result<bool, E> {}

impl<E: Debug> FallibleBool for Result<bool, E> {
    type Err = E;

    fn get(self) -> Result<bool, E> {
        self
    }
}

impl<F, B> Pattern for F where F: FnMut(char) -> B, B: FallibleBool {
    type Err = B::Err;
    fn matches(&mut self, c: char, _: &str) -> Result<bool, Self::Err> {
        self(c).get()
    }
}

pub struct SizedStrPredicate<P: FnMut(&str) -> B, B: FallibleBool> {
    pattern: P,
    len: NonZeroUsize
}

/// Convert `FnMut(&str) -> FalliableBool` into a pattern 
/// by specifying a look-ahead length.
pub trait StrPredicate<B: FallibleBool>: FnMut(&str) -> B + Sized{

    /// Returns a pattern by giving a length hint on a string predicate
    /// 
    /// # Note
    /// 
    /// * All chars above position zero are obtained through peeking
    /// * Do not expect all incoming strings to be length `len`,
    /// the user is expected to handle edge cases.
    /// 
    /// # Panics
    /// 
    /// if `len` is `0`
    fn expecting(self, len: usize) -> SizedStrPredicate<Self, B> {
        let len = NonZeroUsize::new(len)
            .expect("pattern cannot have length 0");
        SizedStrPredicate { pattern: self, len }
    }
}

impl<P: FnMut(&str) -> B, B: FallibleBool> Pattern for SizedStrPredicate<P, B> {
    type Err = B::Err;
    fn matches(&mut self, _: char, s: &str) -> Result<bool, Self::Err> {
        (self.pattern)(s).get()
    }
    fn len(&self) -> NonZeroUsize { self.len }
}


pub struct SizedCharStrPredicate<P: FnMut(char, &str) -> B, B: FallibleBool> {
    pattern: P,
    len: NonZeroUsize
}

/// Convert `FnMut(char, &str) -> FalliableBool` into a pattern 
/// by specifying a look-ahead length.
pub trait CharStrPredicate<B: FallibleBool>: FnMut(char, &str) -> B + Sized{

    /// Returns a pattern by giving a length hint on a string predicate
    /// 
    /// # Note
    /// 
    /// * All chars above position zero are obtained through peeking
    /// * Do not expect all incoming strings to be length `len`,
    /// the user is expected to handle edge cases.
    /// 
    /// # Panics
    /// 
    /// if `len` is `0`
    fn expecting(self, len: usize) -> SizedCharStrPredicate<Self, B> {
        let len = NonZeroUsize::new(len)
            .expect("pattern cannot have length 0");
        SizedCharStrPredicate { pattern: self, len }
    }
}

impl<P: FnMut(char, &str) -> B, B: FallibleBool> Pattern for SizedCharStrPredicate<P, B> {
    type Err = B::Err;
    
    fn matches(&mut self, c: char, s: &str) -> Result<bool, Self::Err> {
        (self.pattern)(c, s).get()
    }
    fn len(&self) -> NonZeroUsize { self.len }
}


pub(crate) struct PatRef<'t, T: Pattern>(pub(crate) &'t mut T);

impl<'t, T: Pattern> Pattern for PatRef<'t, T> {
    type Err = T::Err;
    fn len(&self) -> NonZeroUsize {
        self.0.len()
    }
    fn matches(&mut self, c: char, s: &str) -> Result<bool, Self::Err> {
        self.0.matches(c, s)
    }
    fn sep(&self) -> Sep {
        self.0.sep()
    }
}

#[cfg(feature="std")]
const _:() = {
    extern crate alloc;
    use alloc::boxed::Box;

    impl<E: Debug> Pattern for Box<dyn Pattern<Err = E>> {
        type Err = E;
        #[doc(hidden)]
        fn len(&self) -> NonZeroUsize {
            self.as_ref().len()
        }
        #[doc(hidden)]
        fn matches(&mut self, c: char, s: &str) -> Result<bool, Self::Err> {
            self.as_mut().matches(c, s)
        }
        #[doc(hidden)]
        fn sep(&self) -> Sep {
            self.as_ref().sep()
        }
    }
};

#[cfg(feature = "std")]
const _: () = {
    extern crate alloc;

    use alloc::string::String;

    impl Pattern for String {
        type Err = Never;
        fn matches(&mut self, _: char, s: &str) -> Result<bool, Self::Err> {
            Ok(self == &s)
        }
        fn len(&self) -> NonZeroUsize {
            NonZeroUsize::new(self.chars().count())
                .expect("\"\" is not a valid pattern")
        }
    }

    impl Pattern for &String {
        type Err = Never;
        fn matches(&mut self, _: char, s: &str) -> Result<bool, Self::Err> {
            Ok(self == &s)
        }
        fn len(&self) -> NonZeroUsize {
            NonZeroUsize::new(self.chars().count())
                .expect("\"\" is not a valid pattern")
        }
    }
};


pub struct SepConfig<P: SetSep> {
    pattern: P,
    config: Sep,
}

impl<P: SetSep> Pattern for SepConfig<P> {
    type Err = P::Err;

    fn matches(&mut self, c: char, s: &str) -> Result<bool, Self::Err> {
        self.pattern.matches(c, s)
    }

    fn len(&self) -> NonZeroUsize { self.pattern.len() }

    fn sep(&self) -> Sep {
        self.config
    }
}

/// Allows a pattern to edit its [`sep_method`](crate::Pattern::sep_method).
pub trait SetSep: Pattern + Sized {

    /// Set the [Sep] for this pattern. 
    /// By default this is [Retain](Sep::Retain).
    /// 
    /// # Sep Methods:
    /// 
    /// * [Retain](Sep::Retain): Keep the matched [`char`] in the iterator.
    /// 
    /// * [Yield](Sep::Yield): Yield the matched [`char`] with the output.
    /// 
    /// * [Split](Sep::Split): Discard the matched [`char`].
    /// 
    /// * [Conjoin](Sep::Conjoin): Yield the matched [`char`] while keeping it in the iterator.
    fn sep_with(self, sep: Sep) -> SepConfig<Self> {
        SepConfig { 
            pattern: self, 
            config: sep,
        }
    }
}

impl<P> SetSep for P where P: Pattern + Sized {}

/// Convert a `char` or `&str` match pattern into a `Pattern` 
#[macro_export]
macro_rules! pat {
    ($p: pat) => {
        |c: char| matches!(c, $p)
    };
    (! $p: pat) => {
        |c: char| !matches!(c, $p)
    };
    ($e: expr => $p: pat ) => {
        (|s: str| matches!(c, $p)).expecting($e)
    };
    ($e: expr => ! $p: pat ) => {
        (|s: str| !matches!(c, $p)).expecting($e)
    };
}
