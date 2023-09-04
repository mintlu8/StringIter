use criterion::{Criterion, criterion_main, criterion_group};
use rand::prelude::*;
use string_iter::StringIterable;

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

fn iter_benchmark(c: &mut Criterion){
    let str: String = (0..100).map(|_|gen_char()).collect();
    c.bench_function("chars() to Vec<char>", |b|
        b.iter(|| str.chars().collect::<Vec<_>>())
    );
    c.bench_function("str_iter().chars() to Vec<char>", |b|
        b.iter(|| str.str_iter().chars().collect::<Vec<_>>())
    );
    c.bench_function("str_iter().strs() to Vec<&str>", |b|
        b.iter(|| str.str_iter().strs().collect::<Vec<_>>())
    );
    c.bench_function("str_iter().strs().la(2) to Vec<&str>", |b|
        b.iter(|| str.str_iter().look_ahead(2).collect::<Vec<_>>())
    );
    c.bench_function("str_iter().strs().la(3) to Vec<&str>", |b|
        b.iter(|| str.str_iter().look_ahead(3).collect::<Vec<_>>())
    );
    c.bench_function("chars().rev() to Vec<char>", |b|
        b.iter(|| str.chars().rev().collect::<Vec<_>>())
    );
    c.bench_function("str_iter().chars().rev() to Vec<char>", |b|
        b.iter(|| str.str_iter().chars().rev().collect::<Vec<_>>())
    );
    c.bench_function("str_iter().strs().rev() to Vec<&str>", |b|
        b.iter(|| str.str_iter().strs().rev().collect::<Vec<_>>())
    );
    c.bench_function("chars() to Vec<String>", |b|
        b.iter(|| str.chars().map(|x| x.to_string()). collect::<Vec<_>>())
    );
    c.bench_function("str_iter() to Vec<String>", |b|
        b.iter(|| str.str_iter().map(|x| x.1.to_string()). collect::<Vec<_>>())
    );

    c.bench_function("chars() String::push()", |b|
        b.iter(|| {
            let mut s = String::new();
            str.chars().for_each(|c| s.push(c));
        })
    );
    c.bench_function("str_iter() String::push_str()", |b|
        b.iter(|| {
            let mut s = String::new();
            str.str_iter().for_each(|c| s.push_str(c.1));
        })
    );

}

criterion_group!(benches, iter_benchmark);
criterion_main!(benches);