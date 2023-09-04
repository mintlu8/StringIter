use core::num::NonZeroUsize;

use crate::{pattern::{Pattern, Never}, prelude::Sep};


/// Defines a repeating pattern [`Interval`](crate::patterns::Interval)
/// 
/// `interval!(4)` means every `4` [`char`]s
/// 
/// `interval!(4, 1)` means every `4` [`char`]s,
/// then every `1` [`char`]
/// 
/// `interval!(4 => 3)` means every `4` [`char`]s, 
/// starting from `3`.
/// 
/// `interval!(4 => -10)` means match 10 [`char`]s, 
/// then every `4` [`char`].
/// 
/// `interval!(4, 2 => 1)` means every `4` [`char`]s,
/// then every `2` [`char`]s,
/// starting from `1`
/// 
/// Note that unlike most other patterns, 
/// this uses `[Sep::Yield]` by default.
#[macro_export]
macro_rules! interval {
    ($n: expr) => {
        ::string_iter::patterns::Interval::new(0, [$n])
    };
    ($a: expr, $($b: expr),*) => {
        ::string_iter::patterns::Interval::new(0, [$a, $($b),*])
    };
    ($n: expr => $adv: expr) => {
        ::string_iter::patterns::Interval::new($adv, [$n])
    };
    ($a: expr, $($b: expr),* => $adv: expr) => {
        ::string_iter::patterns::Interval::new($adv, [$a, $($b),*])
    };
}

/// A pattern of substrings with repeating lengths.
/// 
/// See the [`interval`] macro for more information.
pub struct Interval<const N: usize> {
    cursor: isize,
    interval: [NonZeroUsize; N],
}

impl<const N: usize> Interval<N> {
    #[doc(hidden)]
    pub const fn new(mut cursor: isize, lengths: [usize; N]) -> Self {
        assert!(N != 0);
        let mut count = 0;
        let mut sum = 0;
        let mut interval = [NonZeroUsize::MIN; N];
        while count < N {
            assert!(lengths[count] > 0, "expected non-zero length");
            sum += lengths[count];
            interval[count] = match NonZeroUsize::new(sum){
                Some(v) => v,
                None => panic!("expected non-zero length")
            };
            count += 1;
        }
        if cursor.is_positive() {
            cursor = cursor % interval[N-1].get() as isize;
        }
        Self {
            cursor, 
            interval,
        }
    }

    #[doc(hidden)]
    const fn len(&self) -> isize{
        self.interval[N-1].get() as isize
    }
}

impl<const N: usize> Pattern for Interval<N> {
    type Err = Never;

    fn sep(&self) -> Sep {
        // using Retain is not logically consistent here
        Sep::Yield
    }

    fn matches(&mut self, _: char, _: &str) -> Result<bool, Self::Err> {
        self.cursor += 1;
        if self.cursor.is_negative() {
            return Ok(false);
        }
        self.cursor = self.cursor % self.len();
        Ok(self.cursor == 0 || 
            self.interval.iter().any(|x| x.get() == self.cursor as usize))
    }
}