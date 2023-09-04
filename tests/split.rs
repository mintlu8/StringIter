use string_iter::prelude::*;

#[test]
fn split_test(){
    let mut iter = " split this  string please!".str_iter().into_substrs(' ');
    assert!(iter.next().unwrap() == " split");
    assert!(iter.next().unwrap() == " this");
    assert!(iter.next().unwrap() == " ");
    assert!(iter.next().unwrap() == " string");
    assert!(iter.next().unwrap() == " please!");
    assert!(iter.next().is_none());

    let mut iter = " split this  string please!".str_iter().into_substrs(' '.sep_with(Sep::Yield));
    assert!(iter.next().unwrap() == " ");
    assert!(iter.next().unwrap() == "split ");
    assert!(iter.next().unwrap() == "this ");
    assert!(iter.next().unwrap() == " ");
    assert!(iter.next().unwrap() == "string ");
    assert!(iter.next().unwrap() == "please!");
    assert!(iter.next().is_none());

    let mut iter = " split this  string please!".str_iter().into_splits(' ');
    assert!(iter.next().unwrap() == "");
    assert!(iter.next().unwrap() == "split");
    assert!(iter.next().unwrap() == "this");
    assert!(iter.next().unwrap() == "");
    assert!(iter.next().unwrap() == "string");
    assert!(iter.next().unwrap() == "please!");
    assert!(iter.next().is_none());
}


#[test]
fn interval_test(){

    let mut iter = "abcdefg".str_iter().into_substrs(interval!(2));
    assert!(iter.next().unwrap() == "ab");
    assert!(iter.next().unwrap() == "cd");
    assert!(iter.next().unwrap() == "ef");
    assert!(iter.next().unwrap() == "g");
    assert!(iter.next().is_none());

    let mut iter = "abcdefghi".str_iter().into_substrs(interval!(2,1 => 1));
    assert!(iter.next().unwrap() == "a");
    assert!(iter.next().unwrap() == "b");
    assert!(iter.next().unwrap() == "cd");
    assert!(iter.next().unwrap() == "e");
    assert!(iter.next().unwrap() == "fg");
    assert!(iter.next().unwrap() == "h");
    assert!(iter.next().unwrap() == "i");
    assert!(iter.next().is_none());

    let mut iter = "aaaaaaaaaabbcddeff".str_iter().into_substrs(interval!(2,1 => -10));
    assert!(iter.next().unwrap() == "aaaaaaaaaa");
    assert!(iter.next().unwrap() == "bb");
    assert!(iter.next().unwrap() == "c");
    assert!(iter.next().unwrap() == "dd");
    assert!(iter.next().unwrap() == "e");
    assert!(iter.next().unwrap() == "ff");
    assert!(iter.next().is_none());

    let mut iter = "aaa bb ccc dd eee".str_iter().into_splits(interval!(4,3));
    assert!(iter.next().unwrap() == "aaa");
    assert!(iter.next().unwrap() == "bb");
    assert!(iter.next().unwrap() == "ccc");
    assert!(iter.next().unwrap() == "dd");
    assert!(iter.next().unwrap() == "eee");
    assert!(iter.next().is_none());
}