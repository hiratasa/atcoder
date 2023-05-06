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

fn solve(
    i0: usize,
    i1: usize,
    j0: usize,
    j1: usize,
    mi: usize,
    sums: &[Vec<usize>],
    t: usize,
    memo: &mut FxHashMap<(usize, usize, usize, usize, usize), Option<usize>>,
) -> Option<usize> {
    let x = sums[i1][j1] + sums[i0][j0] - sums[i0][j1] - sums[i1][j0];

    if x < (t + 1) * mi {
        return None;
    }

    if t == 0 {
        return Some(x);
    }

    if x == 0 {
        return Some(0);
    }

    if let Some(&r) = memo.get(&(i0, i1, j0, j1, t)) {
        return r;
    }

    let r0 = iproduct!(i0 + 1..i1, 0..t)
        .map(|(i, tt)| {
            solve(i0, i, j0, j1, mi, sums, tt, memo).and_then(|ma| {
                solve(i, i1, j0, j1, mi, sums, t - 1 - tt, memo).map(|ma2| max(ma, ma2))
            })
        })
        .flatten()
        .min();
    let r1 = iproduct!(j0 + 1..j1, 0..t)
        .map(|(j, tt)| {
            solve(i0, i1, j0, j, mi, sums, tt, memo).and_then(|ma| {
                solve(i0, i1, j, j1, mi, sums, t - 1 - tt, memo).map(|ma2| max(ma, ma2))
            })
        })
        .flatten()
        .min();

    let r = it![r0, r1].flatten().min();

    memo.insert((i0, i1, j0, j1, t), r);

    r
}

fn main() {
    let (h, w, t) = read_tuple!(usize, usize, usize);
    let s = read_mat::<usize>(h);

    let mut sums = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            sums[i + 1][j + 1] = s[i][j];
        }
    }
    for i in 1..=h {
        for j in 0..w {
            sums[i][j + 1] += sums[i][j];
        }
    }
    for i in 0..h {
        for j in 1..=w {
            sums[i + 1][j] += sums[i][j];
        }
    }

    let nums = iproduct!((0..=h).tuple_combinations(), (0..=w).tuple_combinations())
        .map(|((i0, i1), (j0, j1))| sums[i1][j1] + sums[i0][j0] - sums[i0][j1] - sums[i1][j0])
        .sorted()
        .dedup()
        .collect::<Vec<_>>();
    let ans = nums
        .citer()
        .filter_map(|mi| {
            solve(0, h, 0, w, mi, &sums, t, &mut FxHashMap::default()).map(|ma| ma - mi)
        })
        .min()
        .unwrap();

    println!("{}", ans);
}
