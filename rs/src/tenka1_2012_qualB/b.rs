#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

// vec with some initial value
#[allow(unused_macros)]
macro_rules! vvec {
    ($($x:expr),+; $y:expr; $n:expr) => {{
        let mut v = vec![$y; $n];

        let mut it = v.iter_mut();
        $(
            *it.next().unwrap() = $x;
        )+

        v
    }}
}

#[allow(unused_macros)]
macro_rules! read_tuple {
    ($($t:ty),+) => {{
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

        let mut it = line.trim()
            .split_whitespace();

        ($(
            it.next().unwrap().parse::<$t>().ok().unwrap()
        ),+)
    }}
}

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_col<T: FromStr>(n: usize) -> Vec<T> {
    (0..n).map(|_| read()).collect()
}

#[allow(dead_code)]
fn read_mat<T: FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_row()).collect()
}

#[allow(dead_code)]
fn read_vec<R, F: FnMut() -> R>(n: usize, mut f: F) -> Vec<R> {
    (0..n).map(|_| f()).collect()
}

trait IteratorDpExt: Iterator + Sized {
    fn dp<T, F: FnMut(&Vec<T>, Self::Item) -> T>(self, init: Vec<T>, mut f: F) -> Vec<T> {
        self.fold(init, |mut dp, item| {
            let next = f(&dp, item);
            dp.push(next);
            dp
        })
    }
}

impl<I> IteratorDpExt for I where I: Iterator + Sized {}

fn parse_camel_case(s: &str) -> Option<(usize, Vec<String>, usize)> {
    s.chars().try_fold(
        (0usize, vec![], 0usize),
        |(leading_us, mut parts, trailing_us), c| {
            if c == '_' {
                if parts.is_empty() {
                    Some((leading_us + 1, parts, trailing_us))
                } else {
                    Some((leading_us, parts, trailing_us + 1))
                }
            } else if trailing_us > 0 {
                None
            } else if c.is_ascii_uppercase() {
                if parts.is_empty() {
                    None
                } else {
                    parts.push(c.to_lowercase().to_string());
                    Some((leading_us, parts, trailing_us))
                }
            } else if c.is_digit(10) {
                if parts.is_empty() {
                    None
                } else {
                    parts.last_mut().unwrap().push(c);
                    Some((leading_us, parts, trailing_us))
                }
            } else {
                if parts.is_empty() {
                    parts.push(c.to_string());
                } else {
                    parts.last_mut().unwrap().push(c)
                }
                Some((leading_us, parts, trailing_us))
            }
        },
    )
}

fn parse_snake_case(s: &str) -> Option<(usize, Vec<String>, usize)> {
    s.chars().try_fold(
        (0usize, vec![], 0usize),
        |(leading_us, mut parts, trailing_us): (usize, Vec<String>, usize), c| {
            if c == '_' {
                if parts.is_empty() {
                    Some((leading_us + 1, parts, trailing_us))
                } else {
                    Some((leading_us, parts, trailing_us + 1))
                }
            } else if trailing_us > 1 {
                None
            } else if c.is_ascii_uppercase() {
                None
            } else if c.is_digit(10) {
                if parts.is_empty() {
                    assert!(trailing_us == 0);
                    None
                } else if trailing_us > 0 {
                    None
                } else {
                    parts.last_mut().unwrap().push(c);
                    Some((leading_us, parts, trailing_us))
                }
            } else {
                if parts.is_empty() {
                    assert!(trailing_us == 0);
                    parts.push(c.to_string());
                    Some((leading_us, parts, trailing_us))
                } else if trailing_us > 0 {
                    parts.push(c.to_string());
                    Some((leading_us, parts, 0))
                } else {
                    parts.last_mut().unwrap().push(c);
                    Some((leading_us, parts, trailing_us))
                }
            }
        },
    )
}

fn construct_camel_case(leading_us: usize, parts: &Vec<String>, trailing_us: usize) -> String {
    chain(
        chain(
            itertools::repeat_n('_', leading_us),
            parts.iter().enumerate().flat_map(|(i, s)| {
                s.chars().enumerate().map(move |(j, c)| {
                    if i > 0 && j == 0 {
                        c.to_ascii_uppercase()
                    } else {
                        c
                    }
                })
            }),
        ),
        itertools::repeat_n('_', trailing_us),
    )
    .collect::<String>()
}

fn construct_snake_case(leading_us: usize, parts: &Vec<String>, trailing_us: usize) -> String {
    chain(
        chain(
            itertools::repeat_n('_', leading_us),
            parts
                .iter()
                .intersperse(&String::from("_"))
                .flat_map(|s| s.chars()),
        ),
        itertools::repeat_n('_', trailing_us),
    )
    .collect::<String>()
}

fn main() {
    let s: String = read();

    if let Some((leading_us, parts, trailing_us)) = parse_camel_case(&s) {
        println!("{}", construct_snake_case(leading_us, &parts, trailing_us));
    } else if let Some((leading_us, parts, trailing_us)) = parse_snake_case(&s) {
        println!("{}", construct_camel_case(leading_us, &parts, trailing_us));
    } else {
        println!("{}", s);
    }
}
