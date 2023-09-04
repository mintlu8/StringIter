use itertools::Itertools;
use rand::Rng;
use string_iter::prelude::*;

/// this generates a unicode char evenly across utf-8 lengths
fn gen_char() -> char {
    let mut rng = rand::thread_rng();
    let byte_size = rng.gen_range(0..4);
    char::from_u32(match byte_size {
        0 => rng.gen_range(0x00..=0x7F),
        1 => rng.gen_range(0x80..=0x7FF),
        2 => rng.gen_range(0x800..=0xFFFF),
        3 => rng.gen_range(0x10000..=0x10FFFF),
        _ => 0,
    }).unwrap_or(char::REPLACEMENT_CHARACTER)
}

trait SubString {
    fn substring(&self, start: usize, end: usize) -> String;
}

impl<T> SubString for T where T: AsRef<str> {
    fn substring(&self, start: usize, end: usize) -> String {
        self.as_ref().chars().skip(start).take(end - start).collect()
    }
}

#[test]
fn empty_test() {
    let original = "";
    assert_eq!(original.str_iter().next(), None);
    assert_eq!(original.str_iter().next_back(), None);
    assert_eq!(original.str_iter().peek(), None);
    assert_eq!(original.str_iter().peek_back(), None);
    assert_eq!(original.str_iter().peekn(0), Ok(""));
    assert_eq!(original.str_iter().peekn_back(0), Ok(""));
    assert_eq!(original.str_iter().peekn(1), Err(""));
    assert_eq!(original.str_iter().peekn_back(1), Err(""));
    assert_eq!(original.str_iter().peekn(2), Err(""));
    assert_eq!(original.str_iter().peekn_back(2), Err(""));
    assert_eq!(original.str_iter().skip_front(1), true);
    assert_eq!(original.str_iter().skip_back(1), true);
}

#[test]
fn iter_join_test(){
    let string: String = (0..100).map(|_|gen_char()).collect();
    
    assert_eq!(string.str_iter().count(), 100);
    assert_eq!(string.str_iter().chars().join(""), string);
    assert_eq!(string.str_iter().strs().join(""), string);

    let string: String = (0..100).map(|_|gen_char()).collect();
    let revstr: String = string.chars().rev().collect();

    assert_eq!(string.str_iter().rev().count(), 100);
    assert_eq!(string.str_iter().chars().rev().join(""), revstr);
    assert_eq!(string.str_iter().strs().rev().join(""), revstr);
}

#[test]
fn peek_test(){
    let string: String = (0..100).map(|_|gen_char()).collect();
    let mut iter = string.str_iter();
    while let Some(x) = iter.peek() {
        assert_eq!(Some(x), iter.next_char())
    }
    assert_eq!(iter.next(), None);

    let string: String = (0..100).map(|_|gen_char()).collect();
    let mut iter = string.str_iter();
    while let Some(x) = iter.peek_back() {
        assert_eq!(Some(x), iter.next_back())
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn peekn_test(){
    let string: String = (0..100).map(|_|gen_char()).collect();
    let mut iter = string.str_iter();

    assert!(iter.peekn(3).ok() == Some(&string.substring(0, 3)));
    assert!(iter.next_slice(3) == Some(&string.substring(0, 3)));

    assert!(iter.peekn(15).ok() == Some(&string.substring(3, 18)));
    assert!(iter.next_slice(15) == Some(&string.substring(3, 18)));

    assert!(iter.peekn(33).ok() == Some(&string.substring(18, 51)));
    assert!(iter.next_slice(33) == Some(&string.substring(18, 51)));
}