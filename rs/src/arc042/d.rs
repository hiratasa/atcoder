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

fn invmod(a: usize, m: usize) -> Option<usize> {
    let (_zero, g, _u, v) =
        std::iter::successors(Some((a as i64, m as i64, 1, 0)), |&(a, b, u, v)| {
            if a == 0 {
                None
            } else {
                Some((b % a, a, -u * (b / a) + v, u))
            }
        })
        .last()
        .unwrap();

    if g == 1 {
        // |v| < m が保障される
        if v < 0 {
            Some((v + m as i64) as usize)
        } else {
            Some(v as usize)
        }
    } else {
        None
    }
}

pub fn pow_mod(mut x: usize, mut p: usize, m: usize) -> usize {
    let mut y = 1;

    x = x % m;
    while p > 0 {
        if p & 1 > 0 {
            y = y * x % m;
        }

        x = x * x % m;
        p >>= 1;
    }

    y
}

// x^i = y mod p となる最小のi>=0を求める
fn log(x: usize, y: usize, p: usize) -> Option<usize> {
    // baby-step giant-step
    // https://tjkendev.github.io/procon-library/python/math/baby-step-giant-step.html
    let m = (p as f64).sqrt() as usize;

    let pows = (0..=m)
        .scan(1, |z, _| Some(replace(z, (*z * x) % p)))
        .collect::<Vec<_>>();

    let pows_map = pows
        .citer()
        .enumerate()
        .map(|(i, z)| (z, i))
        .scan(FxHashSet::default(), |seen, (z, i)| {
            if seen.contains(&z) {
                Some(None)
            } else {
                seen.insert(z);
                Some(Some((z, i)))
            }
        })
        .flatten()
        .collect::<FxHashMap<_, _>>();

    let r = invmod(pows[m], p).unwrap();

    (0..)
        .take_while(|&i| i * m < p - 1)
        .scan(1, |z, i| Some((i, replace(z, (*z * r) % p))))
        .find_map(|(i, z)| pows_map.get(&((y * z) % p)).map(|&j| i * m + j))
}

fn main() {
    let (x, p, a, b) = read_tuple!(usize, usize, usize, usize);

    if (a + p - 2) / (p - 1) * (p - 1) <= b {
        // a <= i <= b を満たす p-1 の倍数が存在
        println!("1");
        return;
    }

    const W: usize = 2000000;

    let ans = if b - a <= W {
        (a..=b).map(|i| pow_mod(x, i, p)).min().unwrap()
    } else {
        // x^-a
        let inv_xa = invmod(pow_mod(x, a, p), p).unwrap();

        (1..)
            .find(|&i| {
                if let Some(l) = log(x, (i * inv_xa) % p, p) {
                    l <= b - a
                } else {
                    false
                }
            })
            .unwrap()
    };

    println!("{}", ans);
}
