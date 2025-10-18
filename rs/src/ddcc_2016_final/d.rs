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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom};
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

fn advance(u: &mut Vec<usize>, t: &[usize], k: usize) {
    u.resize(u.len() + k, usize::MAX);

    let n = t.len();
    let n2 = u.len();
    for i in 0..n2 {
        for j in 1..min(n, n2 - i) {
            u[i + j] = min(u[i + j], u[i] + t[j]);
        }
    }
}

fn calc_impl(t: &[usize], m: usize) -> Vec<usize> {
    let n = t.len();

    if m == 0 {
        t.to_vec()
    } else if m <= n {
        let mut u = t.to_vec();
        advance(&mut u, t, m);

        u[m..m + n].to_vec()
    } else {
        let m0 = (m / 2).saturating_sub(n);
        let mut u = calc_impl(t, m0);
        advance(&mut u, t, t.len() + 2);

        (m..m + n)
            .map(|i| {
                (m0 + (i - m)..m0 + n + (i - m))
                    .map(|j| (j, i - j))
                    .map(|(j, k)| u[j - m0] + u[k - m0])
                    .min()
                    .unwrap()
            })
            .collect()
    }
}

fn calc(t: &[usize], m: usize) -> usize {
    calc_impl(t, m)[0]
}

fn fix(t: &[usize]) -> Vec<usize> {
    let n = t.len();
    let mut u = t.to_vec();

    for i in 0..n {
        for j in 1..n - i {
            u[i + j] = min(u[i + j], u[i] + t[j]);
        }
    }

    u
}

fn main() {
    let (n, m, c) = read_tuple!(usize, usize, usize);
    let ab = read_vec(n, || read_tuple!(usize, usize));

    let k = ab.citer().map(|(a, _b)| a).max().unwrap();

    let ans = (1..=k)
        .map(|i| {
            ab.citer()
                .filter(|&(a, _b)| a >= i)
                .map(|(_a, b)| b)
                .min()
                .unwrap()
        })
        .scan(vec![0], |t, x| {
            t.push(x);
            *t = fix(t);

            Some(t.clone())
        })
        .map(|t| {
            let x = calc(&t, m);

            x + (t.len() - 2) * c
        })
        .min()
        .unwrap();

    println!("{}", ans);
}
