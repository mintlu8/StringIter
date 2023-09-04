use crate::{StringIter, pattern::{Pattern, Never, PatRef, Sep}, prelude::SetSep};

/// If matches are retained, ignore the result on the first element.
struct SplitGuardFirst<P: Pattern>{
    flag: bool,
    pat: P,
}

impl<P: Pattern> SplitGuardFirst<P> {
    fn new(pat: P) -> Self{
        Self {
            flag: !pat.sep().is_retained(),
            pat
        }
    }
}

impl<P: Pattern> Pattern for SplitGuardFirst<P>  {
    type Err = P::Err;

    fn matches(&mut self, c: char, s: &str) -> Result<bool, Self::Err> {
        if self.flag == false {
            // discard the first result
            let _ = self.pat.matches(c, s);
            self.flag = true;
            Ok(false)
        } else {
            self.pat.matches(c, s)
        }
    }

    fn len(&self) -> core::num::NonZeroUsize { self.pat.len() }

    fn sep(&self) -> crate::pattern::Sep { self.pat.sep() }
}
/// If matches are retained, ignore the result on the first element.
struct SplitGuard<P: Pattern>{
    flag: bool,
    pat: P,
}

impl<P: Pattern> SplitGuard<P> {
    fn new(pat: P) -> Self{
        Self {
            flag: !pat.sep().is_retained(),
            pat
        }
    }
}


impl<P: Pattern> Pattern for SplitGuard<P>  {
    type Err = P::Err;

    fn matches(&mut self, c: char, s: &str) -> Result<bool, Self::Err> {
        if self.flag == false {
            // skip the first result
            self.flag = true;
            Ok(false)
        } else {
            self.pat.matches(c, s)
        }
    }

    fn len(&self) -> core::num::NonZeroUsize { self.pat.len() }

    fn sep(&self) -> crate::pattern::Sep { self.pat.sep() }
}

impl<'t> StringIter<'t> {

    /// Split the string into substrings
    /// by repeatedly calling [`next_slice()`](StringIter::next_slice) with a pattern,
    /// while ensuring at least one [`char`] is consumed each call.
    /// 
    /// This iterator does not attempt to remove empty strings, 
    /// use `filter()` for that.
    /// 
    /// # Explanation
    /// 
    /// If ran on the entire string, this can be viewed as:
    /// 
    /// Call `pat.match()` on each char
    /// 
    /// * [`Retain`](Sep::Retain): separate if the next char returns `true`
    /// * [`Yield`](Sep::Yield): separate if this char returns `true`
    /// * [`Split`](Sep::Split): separate if this char returns `true`, and remove this char.
    /// 
    /// # Panics
    /// 
    /// If `pat.sep_method()` is [`Conjoin`](Sep::Conjoin)
    /// 
    /// # Implementation Detail
    /// 
    /// For [`Retain`](Sep::Retain) patterns, 
    /// [`match()`](Pattern::matches) is called on first char, 
    /// but its output is **ignored**.
    pub fn into_substrs(self, pat: impl Pattern<Err=Never>) -> SplitIter<'t, impl Pattern<Err=Never>>{
        if pat.sep() == Sep::Conjoin {
            panic!("Cannot safely split with the conjoined pattern.");
        }
        SplitIter { str: self, pat, count: 0 }
    }

    /// Convenient method for [`into_substrs`](crate::StringIter::into_substrs)
    /// using [`Sep::Split`].
    pub fn into_splits(self, pat: impl Pattern<Err = Never>) -> SplitIter<'t, impl Pattern<Err = Never>>{
        SplitIter { str: self, pat: pat.sep_with(Sep::Split), count: 0 }
    }
}

/// An iterator that yields [`&str`]s 
/// by splitting a [`StringIter`] with a [`Pattern`].
#[derive(Debug, Clone)]
pub struct SplitIter<'t, F: Pattern<Err = Never>>{
    pub(crate) str: StringIter<'t>,
    pub(crate) pat: F,
    pub(crate) count: usize,
}

impl<'t, F> Iterator for SplitIter<'t, F> where F: Pattern<Err = Never>{
    type Item = &'t str;

    fn next(&mut self) -> Option<Self::Item> {
        let pat = PatRef(&mut self.pat);
        if self.count == 0 {
            self.count += 1;
            self.str.next_slice(SplitGuardFirst::new(pat))
        } else {
            self.count += 1;
            self.str.next_slice(SplitGuard::new(pat))
        }
    }
}

impl<'t, F> DoubleEndedIterator for SplitIter<'t, F> where F: Pattern<Err = Never>{
    fn next_back(&mut self) -> Option<Self::Item> {
        let pat = PatRef(&mut self.pat);
        if self.count == 0 {
            self.count += 1;
            self.str.next_slice_back(SplitGuardFirst::new(pat))
        } else {
            self.count += 1;
            self.str.next_slice_back(SplitGuard::new(pat))
        }
    }
}