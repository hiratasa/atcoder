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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
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

trait ToString {
    fn to_string(self: Self) -> String;
}
impl<I, T> ToString for I
where
    I: IntoIterator<Item = T>,
    T: std::convert::TryInto<u32>,
{
    fn to_string(self: Self) -> String {
        self.into_iter()
            .map(|t| t.try_into().ok().unwrap())
            .map(|t| std::convert::TryInto::<char>::try_into(t).ok().unwrap())
            .collect()
    }
}

fn main() {
    let (a, k) = read_tuple!(String, usize);

    let aa = a.parse::<i64>().unwrap();

    let n = a.len();

    const EXTRA: usize = 2;

    let ans = itertools::repeat_n('#', EXTRA)
        .chain(a.chars())
        .scan(
            (BTreeSet::<char>::new(), String::new()),
            |(digits, b), c| {
                if c.is_digit(10) {
                    digits.insert(c);
                    b.push(c);
                } else {
                    b.push('0');
                }

                if k >= 2 && digits.len() <= k - 2 {
                    iproduct!((b'0'..=b'9'), (b'0'..=b'9'))
                        .map(|(d1, d2)| {
                            b.chars()
                                .chain(once(d1 as char))
                                .chain(repeat(d2 as char))
                                .take(n + EXTRA)
                                .to_string()
                                .parse::<i64>()
                                .unwrap()
                        })
                        .map(|x| (x - aa).abs())
                        .min()
                } else if digits.len() == k - 1 {
                    iproduct!((b'0'..=b'9'), (b'0'..=b'9'))
                        .map(|(d1, d2)| (d1 as char, d2 as char))
                        .filter(|&(d1, d2)| {
                            digits.contains(&d1) || digits.contains(&d2) || d1 == d2
                        })
                        .map(|(d1, d2)| {
                            b.chars()
                                .chain(once(d1))
                                .chain(repeat(d2))
                                .take(n + EXTRA)
                                .to_string()
                                .parse::<i64>()
                                .unwrap()
                        })
                        .map(|x| (x - aa).abs())
                        .min()
                } else if digits.len() == k {
                    iproduct!(digits.citer(), digits.citer())
                        .map(|(d1, d2)| {
                            b.chars()
                                .chain(once(d1))
                                .chain(repeat(d2))
                                .take(n + EXTRA)
                                .to_string()
                                .parse::<i64>()
                                .unwrap()
                        })
                        .map(|x| (x - aa).abs())
                        .min()
                } else {
                    None
                }
            },
        )
        .min()
        .unwrap();

    let ans = min(
        ans,
        (aa - repeat(9).take(n - 1).fold(0i64, |x, y| x * 10 + y)).abs(),
    );
    println!("{}", ans);
}
