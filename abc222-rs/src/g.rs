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

fn main() {
    let t = read::<usize>();
    let ns = read_vec(t, || read::<usize>());

    let phi = |n: usize| {
        (2..)
            .scan(n, |m, i| {
                if *m == 1 {
                    None
                } else if i * i > *m {
                    Some(Some(replace(m, 1)))
                } else if *m % i == 0 {
                    *m = iterate(*m, |&k| k / i)
                        .skip_while(|&k| k % i == 0)
                        .next()
                        .unwrap();
                    Some(Some(i))
                } else {
                    Some(None)
                }
            })
            .flatten()
            .fold(n, |m, p| m / p * (p - 1))
    };

    ns.citer()
        .map(|n| {
            if n % 5 == 0 || n % 4 == 0 {
                None
            } else if n == 1 {
                Some(1)
            } else {
                let m = n * 9;
                let m = if m % 2 == 0 { m / 2 } else { m };

                let q = phi(m);

                (1..)
                    .take_while(|&i| i * i <= q)
                    .filter(|&i| q % i == 0)
                    .flat_map(|i| it![i, q / i])
                    .dedup()
                    .filter(|&i| pow_mod(10, i, m) == 1)
                    .min()
            }
        })
        .for_each(|ans| {
            if let Some(ans) = ans {
                println!("{}", ans);
            } else {
                println!("-1");
            }
        })
}
