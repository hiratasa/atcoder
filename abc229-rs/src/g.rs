#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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
use itertools::{chain, iproduct, iterate, izip, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
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
    }
}

#[allow(unused_macros)]
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        bs.buffer_mut()[0] = $x as u64;
        bs
    }};
}

#[allow(unused_macros)]
macro_rules! pushed {
    ($c:expr, $x:expr) => {{
        let mut c = $c;
        c.push($x);
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

fn main() {
    let s = read_str();
    let k = read::<usize>();

    let ypos = s.citer().positions(|c| c == 'Y').collect::<Vec<_>>();
    let m = ypos.len();

    let possum = once(0)
        .chain(ypos.citer())
        .cumsum::<usize>()
        .collect::<Vec<_>>();

    // eprintln!("{:?}", possum);
    let ans = ypos
        .citer()
        .enumerate()
        .map(|(i, pos)| {
            // posの左からj個、右からj-1個集める
            let j0 = lower_bound_int(0, m + 1, |j| {
                if j == 0 {
                    Ordering::Less
                } else if j > i || j - 1 > m - i - 1 {
                    Ordering::Greater
                } else {
                    if (j * ((pos - j) + (pos - 1)) / 2 - (possum[i] - possum[i - j]))
                        + ((possum[i + j - 1 + 1] - possum[i + 1])
                            - (j - 1) * ((pos + j - 1) + (pos + 1)) / 2)
                        <= k
                    {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            }) - 1;

            // posの左からj個、右からj個集める
            let j1 = lower_bound_int(0, m + 1, |j| {
                if j == 0 {
                    Ordering::Less
                } else if j > i || j > m - i - 1 {
                    Ordering::Greater
                } else {
                    if (j * ((pos - j) + (pos - 1)) / 2 - (possum[i] - possum[i - j]))
                        + ((possum[i + j + 1] - possum[i + 1]) - j * ((pos + j) + (pos + 1)) / 2)
                        <= k
                    {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            }) - 1;

            // posの左からj-1個、右からj個集める
            let j2 = lower_bound_int(0, m + 1, |j| {
                if j == 0 {
                    Ordering::Less
                } else if j - 1 > i || j > m - i - 1 {
                    Ordering::Greater
                } else {
                    if ((j - 1) * ((pos - (j - 1)) + pos.saturating_sub(1)) / 2
                        - (possum[i] - possum[i - (j - 1)]))
                        + ((possum[i + j + 1] - possum[i + 1]) - j * ((pos + j) + (pos + 1)) / 2)
                        <= k
                    {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            }) - 1;

            it![2 * j0, 2 * j1 + 1, 2 * j2].max().unwrap()
        })
        .max()
        .unwrap_or(0);

    println!("{}", ans);
}
