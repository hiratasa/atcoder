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
fn println_opt<T: Copy + std::fmt::Display>(ans: Option<T>) {
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

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn main() {
    let (n, a) = read_tuple!(usize, i64);
    let wxv = read_vec(n, || read_tuple!(i64, i64, i64));

    let ans = (0..n)
        .map(|i| {
            let (_, x0, v0) = wxv[i];

            wxv.citer()
                .filter_map(|(w, x, v)| {
                    // x0 + v0 t <= x + vt <= x0 + v0 t + A
                    if v == v0 {
                        if x0 <= x && x <= x0 + a {
                            Some((w, (0, 1), (i64::MAX, 1)))
                        } else {
                            None
                        }
                    } else if v < v0 {
                        // (x - x0 - A) / (v0 - v) <= t <= (x - x0) / (v0 - v)
                        Some((w, (x - x0 - a, v0 - v), (x - x0, v0 - v)))
                    } else {
                        // (x0 - x) / (v - v0) <= t <= (x0 + A - x) / (v - v0)
                        Some((w, (x0 - x, v - v0), (x0 + a - x, v - v0)))
                    }
                })
                .filter(|&(_, _, (d, _))| d >= 0)
                .map(|(w, (b, c), (d, e))| (w, (max(0, b), c), (d, e)))
                .map(|(w, (b, c), (d, e))| {
                    let g0 = gcd(b.abs(), c.abs());
                    let g1 = gcd(d.abs(), e.abs());
                    (w, (b / g0, c / g0), (d / g1, e / g1))
                })
                .flat_map(|(w, (b, c), (d, e))| it![((b, c), w), ((d, e), -w)])
                .sorted_by(|&((b, c), w), &((b2, c2), w2)| {
                    (b.saturating_mul(c2))
                        .cmp(&(b2.saturating_mul(c)))
                        .then(w.cmp(&w2).reverse())
                })
                .group_by(|&((b, c), w)| (b, c, w.signum()))
                .into_iter()
                .map(|(_, it)| it.map(|(_, w)| w).sum::<i64>())
                .cumsum::<i64>()
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("{}", ans);
}
