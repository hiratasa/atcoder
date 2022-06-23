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
fn read_digits() -> Vec<u8> {
    read::<String>()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
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

use num::BigUint;
use num::One;

fn solve0(a: &[u8]) -> usize {
    let k = a.len();
    let b = a.citer().fold(0usize, |x, y| 10 * x + (y as usize));
    (1..=b * b)
        .find(|&x| {
            (x as f64)
                .sqrt()
                .to_string()
                .chars()
                .filter(|&c| c != '.')
                .map(|c| c.to_digit(10).unwrap() as u8)
                .chain(repeat(0))
                .take(k)
                .eq(a.citer())
        })
        .unwrap()
}

fn verify(a: &BigUint, ans: &BigUint) -> bool {
    let a = a.to_radix_be(10);
    let k = a.len();

    if k == 1 && a[0] == 0 {
        return false;
    }
    if k == 0 {
        return false;
    }

    let ans = ans
        * BigUint::from_radix_be(
            &once(1).chain(repeat(0).take(10000)).collect::<Vec<_>>(),
            10,
        )
        .unwrap();

    a.into_iter()
        .eq(ans.sqrt().to_radix_be(10).into_iter().take(k))
}

fn main() {
    let a = read_digits();

    let a = BigUint::from_radix_be(&a, 10).unwrap();
    let a_sq = &a * &a;
    let a1 = &a + BigUint::one();
    let a1_sq = &a1 * &a1;

    let mut b = a_sq.to_radix_le(10);
    let mut b1 = a1_sq.to_radix_le(10);

    let m = b1.len();
    b.resize(m, 0);
    b.reverse();
    b1.reverse();

    assert!(b.len() == b1.len());
    assert!(b1[0] != 0);

    let ans = izip!(b.citer(), b1.citer())
        .scan(0, |c, (d0, d1)| {
            if *c == 0 {
                assert!(d1 >= d0);
                *c = d1 - d0;
                *c = min(*c, 2);
                Some(*c)
            } else {
                assert!(*c * 10 + d1 > d0);
                *c = *c * 10 + d1 - d0;
                *c = min(*c, 2);
                Some(*c)
            }
        })
        .enumerate()
        .find_map(|(i, x)| {
            if (m - i - 1) % 2 > 0 {
                None
            } else if b[i + 1..].citer().all(|c| c == 0) {
                Some(b[..=i].citer().collect::<Vec<_>>())
            } else if x == 0 {
                None
            } else if x == 1 && b1[i + 1..].citer().all(|c| c == 0) {
                None
            } else {
                Some(
                    (BigUint::from_radix_be(&b[..=i], 10).unwrap() + BigUint::one())
                        .to_radix_be(10),
                )
            }
        })
        .unwrap();

    println!("{}", ans.citer().skip_while(|&c| c == 0).join(""));
}
