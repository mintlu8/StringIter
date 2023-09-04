
use crate::{StringIter, pattern::{Pattern, Never, Sep}};



impl Sep {
    #[inline]
    pub(crate) fn is_yielded(&self) -> bool {
        matches!(self, Sep::Yield|Sep::Conjoin)
    }

    #[inline]
    pub(crate) fn is_retained(&self) -> bool {
        matches!(self, Sep::Retain|Sep::Conjoin)
    }
}


impl<'t> StringIter<'t> {

    /// Obtain the contents of the StringIter, leaving it empty.
    pub fn drain(&mut self) -> &'t str {
        let result = self.str;
        let len = self.len();
        // to be extra safe with merge() in the crate
        self.str = &self.str[len..len];
        result
    }

    /// Gets a slice out of a StringIter
    /// using a fallible pattern. 
    /// 
    /// If Err is [`Never`], you can use 
    /// [the infallible version](crate::StringIter::next_slice) instead.
    /// 
    /// The iterator will not be changed if the match fails.
    /// 
    /// # Pattern
    /// 
    /// see [supported patterns](crate#supported-patterns)
    /// 
    /// # Pattern Configuration
    /// 
    /// see [`SetSep`](crate::SetSep)
    pub fn try_next_slice<P: Pattern>(&mut self, mut pat: P) -> Result<Option<&'t str>, P::Err> {
        if self.len() == 0{
            return Ok(None);
        }
        let mut index = self.len();
        let mut char_len = 0;
        if pat.len().get() == 1{
            for (c, s) in self.clone(){
                if pat.matches(c, s)? {
                    index = s.as_ptr() as usize - self.str.as_ptr() as usize;
                    char_len = s.len();
                    break;
                }
            }
        } else {
            for (c, s) in self.clone().look_ahead(pat.len().get()){
                if pat.matches(c, s)? {
                    index = s.as_ptr() as usize - self.str.as_ptr() as usize;
                    char_len = c.len_utf8();
                    break;
                }
            }
        }
        unsafe{
            let result = if pat.sep().is_yielded() {
                self.str.get_unchecked(..index + char_len)
            } else {
                self.str.get_unchecked(..index)
            };
            if pat.sep().is_retained() {
                self.str = self.str.get_unchecked(index..);
            } else {
                self.str = self.str.get_unchecked(index + char_len..);
            }
            Ok(Some(result))
        }
    }

    /// Gets a slice from a StringIter in reverse,
    /// using a fallible pattern. 
    /// 
    /// If Err is [`Never`], you can use 
    /// [the infallible version](crate::StringIter::next_slice_back) instead.
    /// 
    /// The iterator will not be changed if the match fails.
    /// 
    /// See [try_next_slice](crate::StringIter::try_next_slice)
    pub fn try_next_slice_back<P: Pattern>(&mut self, mut pat: P) -> Result<Option<&'t str>, P::Err> {
        if self.len() == 0{
            return Ok(None);
        }
        let mut index = self.len();
        let mut char_len = 0;
        if pat.len().get() == 1{
            for (c, s) in self.clone().rev(){
                if pat.matches(c, s)? {
                    index = s.as_ptr() as usize - self.str.as_ptr() as usize;
                    char_len = s.len();
                    break;
                }
            }
        } else {
            for (c, s) in self.clone().look_ahead(pat.len().get()).rev(){
                if pat.matches(c, s)? {
                    index = s.as_ptr() as usize - self.str.as_ptr() as usize;
                    char_len = c.len_utf8();
                    break;
                }
            }
        }
        for (c, s) in self.clone().rev(){
            if pat.matches(c, s)? {
                index = s.as_ptr() as usize - self.str.as_ptr() as usize;
                char_len = s.len();
                break;
            }
        }
        unsafe{
            let result = if pat.sep().is_yielded() {
                self.str.get_unchecked(index..)
            } else {
                self.str.get_unchecked(index + char_len..)
            };
            if pat.sep().is_retained() {
                self.str = self.str.get_unchecked(..index + char_len);
            } else {
                self.str = self.str.get_unchecked(..index);
            }
            Ok(Some(result))
        }
    }

    /// Gets a slice from a StringIter,
    /// using a non-fallible pattern. 
    /// 
    /// See [try_next_slice](crate::StringIter::try_next_slice)
    #[inline]
    pub fn next_slice<P: Pattern<Err = Never>> (&mut self, pat: P) -> Option<&'t str> {
        self.try_next_slice(pat).unwrap()
    }


    /// Gets a slice from a StringIter in reverse,
    /// using a non-fallible pattern. 
    /// 
    /// See [try_next_slice](crate::StringIter::try_next_slice)
    #[inline]
    pub fn next_slice_back<P: Pattern<Err = Never>> (&mut self, pat: P) -> Option<&'t str> {
        self.try_next_slice_back(pat).unwrap()
    }
}