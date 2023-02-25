#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64;
#[allow(unused_imports)]
use std::i64;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::iter::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom, Rng, SeedableRng};
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;

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
macro_rules! it {
    ($x:expr) => {
        once($x)
    };
    ($first:expr,$($x:expr),+) => {
        chain(
            once($first),
            it!($($x),+)
        )
    };
    ($($x:expr),+,) => {
        it![$($x),+]
    };
}

#[allow(unused_macros)]
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        if $n > 0 {
            bs.buffer_mut()[0] = $x as u64;
        }
        bs
    }};
}

#[allow(unused_macros)]
macro_rules! pushed {
    ($c:expr, $x:expr) => {{
        let x = $x;
        let mut c = $c;
        c.push(x);
        c
    }};
}

#[allow(unused_macros)]
macro_rules! inserted {
    ($c:expr, $($x:expr),*) => {{
        let mut c = $c;
        c.insert($($x),*);
        c
    }};
}

#[allow(unused_macros)]
macro_rules! read_tuple {
    ($($t:ty),+) => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

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
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_digits() -> Vec<usize> {
    read::<String>()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

#[allow(dead_code)]
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

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

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

trait IterCopyExt<'a, T>: IntoIterator<Item = &'a T> + Sized
where
    T: 'a + Copy,
{
    fn citer(self) -> std::iter::Copied<Self::IntoIter> {
        self.into_iter().copied()
    }
}

impl<'a, T, I> IterCopyExt<'a, T> for I
where
    I: IntoIterator<Item = &'a T>,
    T: 'a + Copy,
{
}

#[allow(dead_code)]
fn lower_bound<T, F>(mut begin: T, mut end: T, epsilon: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    let two = T::try_from(2).ok().unwrap();
    while end - begin >= epsilon {
        let mid = begin + (end - begin) / two;
        match f(mid) {
            std::cmp::Ordering::Less => {
                begin = mid + epsilon;
            }
            _ => {
                end = mid;
            }
        }
    }
    begin
}
#[allow(dead_code)]
fn lower_bound_int<T, F>(begin: T, end: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    lower_bound(begin, end, T::try_from(1).ok().unwrap(), f)
}

fn solve0(k: usize, s: &[char]) -> usize {
    let n = s.len();

    (0usize..1 << n)
        .filter(|&t| t.count_ones() as usize == k)
        .map(|t| {
            let mut ss = s.to_vec();

            for i in 0..n {
                if t & (1 << i) > 0 {
                    ss[i] = if ss[i] == 'X' { 'Y' } else { 'X' };
                }
            }

            ss
        })
        .map(|ss| {
            ss.citer()
                .tuple_windows()
                .filter(|&(c0, c1)| c0 == 'Y' && c1 == 'Y')
                .count()
        })
        .max()
        .unwrap()
}

fn main() {
    let (n, k) = read_tuple!(usize, usize);
    let s = read_str();

    if s.citer().all(|c| c == 'X') {
        println!("{}", k.saturating_sub(1));
        return;
    }
    if s.citer().all(|c| c == 'Y') {
        println!("{}", (n - k).saturating_sub(1));
        return;
    }

    let blocks = s
        .citer()
        .enumerate()
        .group_by(|&(_, c)| c)
        .into_iter()
        .filter(|(c, _)| *c == 'X')
        .map(|(_, it)| {
            let v = it.collect::<Vec<_>>();

            let edge = v.citer().any(|(i, _)| i == 0 || i == n - 1);

            (edge, v.len())
        })
        .sorted()
        .collect::<Vec<_>>();

    let ans = if k <= blocks.citer().map(|(_, i)| i).sum::<usize>() {
        s.citer()
            .tuple_windows()
            .filter(|&(c0, c1)| c0 == 'Y' && c1 == 'Y')
            .count()
            + blocks
                .citer()
                .scan(k, |kk, (edge, b)| {
                    if *kk == 0 {
                        None
                    } else if *kk < b {
                        Some(replace(kk, 0))
                    } else {
                        *kk -= b;

                        if edge {
                            Some(b)
                        } else {
                            Some(b + 1)
                        }
                    }
                })
                .sum::<usize>()
    } else {
        let ky = k - blocks.citer().map(|(_, b)| b).sum::<usize>();

        let y_blocks = s
            .citer()
            .enumerate()
            .group_by(|&(_, c)| c)
            .into_iter()
            .filter(|(c, _)| *c == 'Y')
            .map(|(_, it)| {
                let v = it.collect::<Vec<_>>();

                let edge = v.citer().any(|(i, _)| i == 0 || i == n - 1);

                (edge, v.len())
            })
            .sorted_by_key(|&(edge, b)| (!edge, Reverse(b)))
            .collect::<Vec<_>>();

        n - 1
            - y_blocks
                .citer()
                .scan(ky, |kk, (edge, b)| {
                    if *kk == 0 {
                        None
                    } else if *kk < b {
                        if edge {
                            Some(replace(kk, 0))
                        } else {
                            Some(replace(kk, 0) + 1)
                        }
                    } else if edge {
                        *kk -= b;
                        Some(b)
                    } else {
                        *kk -= b;
                        Some(b + 1)
                    }
                })
                .sum::<usize>()
    };

    println!("{}", ans);
}
