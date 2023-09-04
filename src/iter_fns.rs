use crate::{StringIter, pattern::{Pattern, Never}};


macro_rules! get {
    ($e: expr, $v: expr) => {
        *$e.get_unchecked($v) as u32
    };
}

const B6: u32 = 0b111_111;
const B5: u32 = 0b11_111;
const B4: u32 = 0b1_111;
const B3: u32 = 0b111;


/// SAFETY: s contains a sigle byte UTF-8 code point
#[inline]
unsafe fn s2c1(s: &str) -> char {
    let s = s.as_bytes();
    char::from_u32_unchecked(get!(s, 0))
}

/// SAFETY: s contains a 2 byte UTF-8 code point
#[inline]
unsafe fn s2c2(s: &str) -> char {
    let s = s.as_bytes();
    char::from_u32_unchecked(
        (get!(s, 0) & B5) << 6 | get!(s, 1) & B6
    )
}

/// SAFETY: s contains a 3 byte UTF-8 code point
#[inline]
unsafe fn s2c3(s: &str) -> char {
    let s = s.as_bytes();
    char::from_u32_unchecked(
        ((get!(s, 0) & B4) << 6 
        | get!(s, 1) & B6) << 6 
        | get!(s, 2) & B6
    )
}

/// SAFETY: s contains a 4 byte UTF-8 code point
#[inline]
unsafe fn s2c4(s: &str) -> char {
    let s = s.as_bytes();
    char::from_u32_unchecked(
        (((get!(s, 0) & B3) << 6
        | get!(s, 1) & B6) << 6
        | get!(s, 2) & B6) << 6
        | get!(s, 3) & B6
    )
}

impl<'t> StringIter<'t> {

    /// Returns a leading [`char`] and its [`&str`](str) representation
    /// from the `StringIter` and advances it.
    ///
    /// This is the same as `StringIter::next()`,
    /// but available to mapped versions of `StringIter`.
    #[inline]
    pub fn next_char(&mut self) -> Option<(char, &'t str)> {
        let x = *self.str.as_bytes().get(0)?;
        // SAFETY: safe since self.str is valid utf-8
        unsafe{
            if x < 0x80 {
                let result = self.str.get_unchecked(..1);
                self.str = self.str.get_unchecked(1..);
                Some((s2c1(result), result))
            } else if x < 0xE0 {
                let result = self.str.get_unchecked(..2);
                self.str = self.str.get_unchecked(2..);
                Some((s2c2(result), result))
            } else if x < 0xF0 {
                let result = self.str.get_unchecked(..3);
                self.str = self.str.get_unchecked(3..);
                Some((s2c3(result), result))
            } else {
                let result = self.str.get_unchecked(..4);
                self.str = self.str.get_unchecked(4..);
                Some((s2c4(result), result))
            }
        }
    }

    /// Returns a trailing [`char`] and its [`&str`](str) representation
    /// from the `StringIter` and advances it.
    ///
    /// This is the same as `StringIter::next_back()`,
    /// but available to mapped versions of `StringIter`.
    #[inline]
    pub fn next_char_back(&mut self) -> Option<(char, &'t str)> {
        let bytes = self.str.as_bytes();
        if bytes.is_empty() {
            return None;
        }
        let mut index = bytes.len() - 1;
        // SAFETY: safe since self.str is valid utf-8
        unsafe{
            if *bytes.get_unchecked(index) < 0x80 {
                let result = self.str.get_unchecked(index..);
                self.str = self.str.get_unchecked(..index);
                return Some((s2c1(result), result))
            }
            index -= 1;
            if *bytes.get_unchecked(index) & 0b1100_0000 == 0b1100_0000 {
                let result = self.str.get_unchecked(index..);
                self.str = self.str.get_unchecked(..index);
                return Some((s2c2(result), result))
            }
            index -= 1;
            if *bytes.get_unchecked(index) & 0b1110_0000 == 0b1110_0000 {
                let result = self.str.get_unchecked(index..);
                self.str = self.str.get_unchecked(..index);
                return Some((s2c3(result), result))
            }
            index -= 1;
            let result = self.str.get_unchecked(index..);
            self.str = self.str.get_unchecked(..index);
            return Some((s2c4(result), result))
        }
    }

    /// Returns a leading [`char`] and its [`&str`](str) representation
    /// from the `StringIter` without advancing it.
    #[inline]
    pub fn peek(&self) -> Option<(char, &'t str)> {
        let x = *self.str.as_bytes().first()?;
        // SAFETY: safe since self.str is valid utf-8
        unsafe{
            if x < 0x80 {
                let result = self.str.get_unchecked(..1);
                Some((s2c1(result), result))
            } else if x < 0xE0 {
                let result = self.str.get_unchecked(..2);
                Some((s2c2(result), result))
            } else if x < 0xF0 {
                let result = self.str.get_unchecked(..3);
                Some((s2c3(result), result))
            } else {
                let result = self.str.get_unchecked(..4);
                Some((s2c4(result), result))
            }
        }
    }

    /// Returns a trailing [`char`] and its [`&str`](str) representation
    /// from the `StringIter` without advancing it.
    #[inline]
    pub fn peek_back(&self) -> Option<(char, &'t str)> {
        let bytes = self.str.as_bytes();
        if bytes.is_empty() {
            return None;
        }
        let mut index = bytes.len() - 1;
        // SAFETY: safe since self.str is valid utf-8
        unsafe{
            if *bytes.get_unchecked(index) < 0x80 {
                let result = self.str.get_unchecked(index..);
                return Some((s2c1(result), result))
            }
            index -= 1;
            if *bytes.get_unchecked(index) & 0b1100_0000 == 0b1100_0000 {
                let result = self.str.get_unchecked(index..);
                return Some((s2c2(result), result))
            }
            index -= 1;
            if *bytes.get_unchecked(index) & 0b1110_0000 == 0b1110_0000 {
                let result = self.str.get_unchecked(index..);
                return Some((s2c3(result), result))
            }
            index -= 1;
            let result = self.str.get_unchecked(index..);
            return Some((s2c4(result), result))
        }
    }


    /// Returns leading [`&str`](str)s with maximum count `n` 
    /// from the `StringIter` without advancing it.
    ///
    /// Returns OK if n [`char`]s found, Err if less than n [`char`]s found.
    pub fn peekn(&self, n: usize) -> Result<&'t str, &'t str> {
        let mut index = 0usize;
        for _ in 0..n {
            let x = *self.str.as_bytes().get(index)
                .ok_or_else(|| unsafe {
                    self.str.get_unchecked(..index)
                })?;
            if x < 0x80{
                index += 1;
            } else if x < 0xE0 {
                index += 2;
            } else if x < 0xF0 {
                index += 3;
            } else {
                index += 4;
            }
        }
        // SAFETY: this is safe because self is valid utf-8
        unsafe {
            Ok(self.str.get_unchecked(..index))
        }
    }

    /// Returns trailing [`&str`](str)s with maximum count `n` 
    /// from the `StringIter` without advancing it.
    ///
    /// Returns OK if n [`char`]s found, Err if less than n [`char`]s found.
    pub fn peekn_back(&self, n: usize) -> Result<&'t str, &'t str> {
        let bytes = self.str.as_bytes();
        let mut index = bytes.len();
        // SAFETY: this is safe because self is valid utf-8
        unsafe{
            for _ in 0..n {
                if index == 0 {
                    return Err(self.str.get_unchecked(index..));
                }
                index -= 1;
                if *bytes.get_unchecked(index) < 0x80 {
                    continue;
                }
                index -= 1;
                if *bytes.get_unchecked(index) & 0b1100_0000 == 0b1100_0000 {
                    continue;
                }
                index -= 1;
                if *bytes.get_unchecked(index) & 0b1110_0000 == 0b1110_0000 {
                    continue;
                }
                index -= 1;
            }
            Ok(self.str.get_unchecked(index..))
        }
    }


    /// Removes leading and trailing [`char`]s that matches a `Pattern` from the `StringIter`.
    pub fn trim_by(&mut self, f: impl Pattern<Err = Never> + Clone){
        self.trim_start_by(f.clone());
        self.trim_end_by(f);
    }

    /// Removes leading [`char`]s that matches a `Pattern` from the `StringIter`.
    pub fn trim_start_by(&mut self, mut f: impl Pattern<Err = Never>){
        let bytes = self.as_bytes();
        let mut index = 0;
        while let Some(x) = bytes.get(index) {
            let x = *x;
            // SAFETY: safe since self.str is valid utf-8
            let (c, s, len) = unsafe {
                if x < 0x80 {
                    let result = self.str.get_unchecked(index..index+1);
                    (s2c1(result), result, 1)
                } else if x < 0xE0 {
                    let result = self.str.get_unchecked(index..index+2);
                    (s2c2(result), result, 2)
                } else if x < 0xF0 {
                    let result = self.str.get_unchecked(index..index+3);
                    (s2c3(result), result, 3)
                } else {
                    let result = self.str.get_unchecked(index..index+4);
                    (s2c4(result), result, 4)
                }
            };
            if f.matches(c, s).unwrap() {
                index += len;
            } else {
                break;
            }
        }
        unsafe {
            self.str = self.str.get_unchecked(index..)
        }
    }

    /// Removes trailing [`char`]s that matches a `Pattern` from the `StringIter`.
    pub fn trim_end_by(&mut self, mut f: impl Pattern<Err = Never>){
        let bytes = self.as_bytes();
        let mut index = self.len() - 1;
        while let Some(x) = bytes.get(index) {
            let mut i = index;
            // SAFETY: safe since self.str is valid utf-8
            unsafe {
                i -= 1;
                if *x < 0x80 {
                    let s = self.str.get_unchecked(i..index);
                    if !f.matches(s2c1(s), s).unwrap() {
                        break;
                    } else {
                        index = i;
                        continue;
                    }
                }
                i -= 1;
                if *bytes.get_unchecked(i) & 0b1100_0000 == 0b1100_0000 {
                    let s = self.str.get_unchecked(i..index);
                    if !f.matches(s2c2(s), s).unwrap() {
                        break;
                    } else {
                        index = i;
                        continue;
                    }
                }
                i -= 1;
                if *bytes.get_unchecked(i) & 0b1110_0000 == 0b1110_0000  {
                    let s = self.str.get_unchecked(i..index);
                    if !f.matches(s2c3(s), s).unwrap() {
                        break;
                    } else {
                        index = i;
                        continue;
                    }
                }
                i -= 1;
                let s = self.str.get_unchecked(i..index);
                if !f.matches(s2c4(s), s).unwrap() {
                    break;
                } else {
                    index = i;
                    continue;
                }
            }
        }
        unsafe {
            self.str = self.str.get_unchecked(..index)
        }
    }
}