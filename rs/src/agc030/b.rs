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

fn main() {
    let (l, n) = read_tuple!(usize, usize);
    let x = read_col::<usize>(n);

    if n == 1 {
        println!("{}", max(x[0], l - x[0]));
        return;
    }

    let csum = once(0)
        .chain(x.citer())
        .cumsum::<usize>()
        .collect::<Vec<_>>();
    let rcsum = once(0)
        .chain(x.citer().rev().map(|t| l - t))
        .cumsum::<usize>()
        .collect::<Vec<_>>();

    let ans = (0..n)
        .map(|i| {
            let j = n - 1 - i;

            // 最初左、最後左
            let r0 = if i > 0 && (i > 1 || j == 0) {
                let m = min(i - 1, j);

                2 * (csum[i - 1] - csum[i - 1 - m])
                    + 2 * (rcsum[j] - rcsum[j - m])
                    + x[i - 1]
                    + max(x[i] - x[i - 1], x[i - 1] + (l - x[i]))
            } else {
                0
            };

            // 最初左、最後右
            let r1 = if i > 0 && j > 0 {
                let m = min(i - 1, j - 1);

                2 * (csum[i] - csum[i - 1 - m])
                    + 2 * (rcsum[j - 1] - rcsum[j - 1 - m])
                    + (l - x[i + 1])
                    + max(x[i + 1] - x[i], (l - x[i + 1]) + x[i])
            } else {
                0
            };

            // 最初右、最後右
            let r2 = if j > 0 && (j > 1 || i == 0) {
                let m = min(i, j - 1);

                2 * (csum[i] - csum[i - m])
                    + 2 * (rcsum[j - 1] - rcsum[j - 1 - m])
                    + (l - x[i + 1])
                    + max(x[i + 1] - x[i], (l - x[i + 1]) + x[i])
            } else {
                0
            };

            // 最初右、最後左
            let r3 = if i > 0 && j > 0 {
                let m = min(i - 1, j - 1);

                2 * (csum[i - 1] - csum[i - 1 - m])
                    + 2 * (rcsum[j] - rcsum[j - 1 - m])
                    + x[i - 1]
                    + max(x[i] - x[i - 1], x[i - 1] + (l - x[i]))
            } else {
                0
            };

            max(max(r0, r1), max(r2, r3))
        })
        .max()
        .unwrap();

    println!("{}", ans);
}
