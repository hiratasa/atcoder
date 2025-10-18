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

fn suffix_ranks<T: Ord>(s: &[T]) -> Vec<Vec<usize>> {
    let n = s.len();
    let sa0 = (0..n).sorted_by_key(|&i| &s[i]).collect_vec();
    let r0 = sa0
        .iter()
        .group_by(|&&i| &s[i])
        .into_iter()
        .enumerate()
        .fold(vec![0; n], |mut r, (rank, (_, it))| {
            for &idx in it {
                r[idx] = rank;
            }
            r
        });

    successors(Some((r0, 1)), |(prev_r, prev_len)| {
        if *prev_len >= n {
            return None;
        }

        let len = 2 * *prev_len;
        let to_key = |i: usize| (prev_r.get(i), prev_r.get(i + len / 2));
        let sa = (0..n).sorted_by_key(|&i| to_key(i)).collect_vec();
        let r = sa
            .iter()
            .group_by(|&&i| to_key(i))
            .into_iter()
            .enumerate()
            .fold(vec![0; n], |mut r, (rank, (_, it))| {
                for &idx in it {
                    r[idx] = rank;
                }
                r
            });
        Some((r, len))
    })
    .map(|t| t.0)
    .collect_vec()
}

fn is_prefix_of(sr: &Vec<Vec<usize>>, idx0: usize, idx1: usize) -> bool {
    let n = sr[0].len();

    if idx0 < idx1 {
        return false;
    }

    (0..sr.len())
        .rev()
        .try_fold((idx0, idx1), |(idx0, idx1), i| {
            let len = 1 << i;

            if len > n - idx0 {
                Some((idx0, idx1))
            } else if sr[i][idx0] == sr[i][idx1] {
                Some((idx0 + len, idx1 + len))
            } else {
                None
            }
        })
        .is_some()
}

fn main() {
    let n: usize = read();
    let s = read_str();

    let sr = suffix_ranks(&s);
    let sa = &sr[sr.len() - 1];

    (0..n)
        .sorted_by(|&i, &j| {
            if i > j && is_prefix_of(&sr, i, j) {
                sa[j].cmp(&sa[j + (n - i)])
            } else if i < j && is_prefix_of(&sr, j, i) {
                sa[i + (n - j)].cmp(&sa[i])
            } else {
                sa[i].cmp(&sa[j])
            }
        })
        .for_each(|i| println!("{}", i + 1));
}
