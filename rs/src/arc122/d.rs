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
use itertools::{Itertools, chain, iproduct, iterate, izip};
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

fn min_pair(a: &[usize], b: &[usize], idx: u32) -> usize {
    if idx == 0 {
        return 0;
    }

    if a.is_empty() || b.is_empty() {
        return usize::MAX;
    }

    let a0 = a
        .citer()
        .filter(|&x| x & (1 << (idx - 1)) == 0)
        .collect::<Vec<_>>();
    let a1 = a
        .citer()
        .filter(|&x| x & (1 << (idx - 1)) > 0)
        .collect::<Vec<_>>();
    let b0 = b
        .citer()
        .filter(|&x| x & (1 << (idx - 1)) == 0)
        .collect::<Vec<_>>();
    let b1 = b
        .citer()
        .filter(|&x| x & (1 << (idx - 1)) > 0)
        .collect::<Vec<_>>();

    if (!a0.is_empty() && !b0.is_empty()) || (!a1.is_empty() && !b1.is_empty()) {
        min(min_pair(&a0, &b0, idx - 1), min_pair(&a1, &b1, idx - 1))
    } else {
        min(min_pair(&a0, &b1, idx - 1), min_pair(&a1, &b0, idx - 1)) + (1 << (idx - 1))
    }
}

fn solve(a: &[usize], mask: usize) -> usize {
    if a.is_empty() {
        return 0;
    }

    let ma = a.citer().map(|x| x & mask).max().unwrap();

    if ma == 0 {
        return 0;
    }

    let idx = (ma + 1).next_power_of_two().trailing_zeros() - 1;

    let a0 = a
        .citer()
        .filter(|&x| x & (1 << idx) == 0)
        .collect::<Vec<_>>();
    let a1 = a
        .citer()
        .filter(|&x| x & (1 << idx) > 0)
        .collect::<Vec<_>>();

    if a1.len() % 2 == 0 {
        max(solve(&a0, (1 << idx) - 1), solve(&a1, (1 << idx) - 1))
    } else {
        min_pair(&a0, &a1, idx) + (1 << idx)
    }
}

fn main() {
    let n = read::<usize>();
    let a = read_row::<usize>();

    let ans = solve(&a, (1 << 30) - 1);

    println!("{}", ans);
}
