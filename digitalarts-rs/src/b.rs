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
use std::iter::*;
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

fn main() {
    let s = read::<String>();

    let ans = if s == "a" {
        String::from("NO")
    } else if s == repeat_n('z', 20).collect::<String>() {
        String::from("NO")
    } else if s.chars().all_equal() {
        if s.len() == 1 {
            let c = s.chars().next().unwrap();
            let mut t = String::new();
            t.push((c as u8 - 1) as char);
            t.push('a');
            t
        } else {
            match s.chars().next().unwrap() {
                'a' => chain(s.chars().skip(2), once('b')).collect::<String>(),
                'z' => chain(s.chars().skip(1), "ya".chars()).collect::<String>(),
                c @ _ => chain(
                    s.chars().skip(2),
                    chain(once(c as u8 - 1), once(c as u8 + 1)).map_into::<char>(),
                )
                .collect::<String>(),
            }
        }
    } else {
        let t = s.chars().sorted().collect::<String>();
        let u = s.chars().sorted_by_key(|&c| Reverse(c)).collect::<String>();

        if s == t {
            u
        } else {
            t
        }
    };

    println!("{}", ans);
}
