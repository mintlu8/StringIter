use core::iter::Peekable;

/// See documentation in [`StringExt`]
pub(crate) fn merge<'t>(parent: &'t str, first: &str, second: &str) -> Option<&'t str> {
    let st = first.as_ptr() as usize;
    if st as usize + first.len() == second.as_ptr() as usize {
        let start = st.checked_sub(parent.as_ptr() as usize)?;
        let end = start + first.len() + second.len();
        if end > parent.len() {
            return None;
        }
        // SAFETY: safe, since start and end are inside from.len()
        // and enclose valid utf-8 strings first and second
        Some(unsafe {
            &parent.get_unchecked(start..end)
        })
    } else {
        None
    }
}

/// Iterators for merging substrings.
/// 
/// The parent string is needed for satefy.
pub trait Merge<'t>: Iterator<Item = &'t str> + Sized {
    /// Merge an iterator of adjacent [`&str`](str)s in `parent` into a single &str
    /// 
    /// Returns `None` if some [`&str`](str)s are not adjacent
    fn merge_all(mut self, parent: &'t str) -> Option<&'t str>{
        let first = self.next();
        self.fold(first, |x, y| merge(parent, x?, y))
    }

    /// Merge adjacent pairs of [`&str`](str)'s in `parent` by a predicate, 
    /// 
    /// Function `f` only gets called on adjacent substrings.
    /// 
    /// # Example
    /// 
    /// Merging all adjacent substrings:
    /// ```
    /// # use string_iter::Merge;
    /// let string = "foobarbaz quxquux";
    /// let substrs = [&string[0..3], &string[3..6], &string[6..9], 
    ///     &string[10..13], &string[13..17]];
    /// let mut iter = substrs.into_iter().merge_by(string, |_, _| true);
    /// assert_eq!(iter.next(), Some("foobarbaz"));
    /// assert_eq!(iter.next(), Some("quxquux"));
    /// ```
    /// 
    fn merge_by<F: FnMut(&str, &str) -> bool>(self, parent: &'t str, f: F) -> MergeIter<'t, Self, F>{
        MergeIter { parent, iter: self.peekable(), predicate: f }
    }
}

impl<'t, T> Merge<'t> for T where T: Iterator<Item = &'t str> {}


/// An iterator for merging substrings.
pub struct MergeIter<'t, T: Iterator<Item = &'t str>, F: FnMut(&str, &str) -> bool>{
    parent: &'t str,
    iter: Peekable<T>,
    predicate: F,
}

impl<'t, T, F> Iterator for MergeIter<'t, T, F>
    where T: Iterator<Item = &'t str>, F: FnMut(&str, &str) -> bool{

    type Item = &'t str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut curr = self.iter.next()?;
        let mut last = curr;
        while let Some(next) = self.iter.peek() {
            if let Some(merged) = merge(self.parent, curr, next) {
                if (self.predicate)(last, next) {
                    curr = merged;
                    last = next;
                    self.iter.next();
                }
            } else {
                break;
            }
        }
        Some(curr)
    }
}
