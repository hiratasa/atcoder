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

fn suffix_array_r<T: Ord>(s: &[T]) -> Vec<usize> {
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

    iterate(2, |len| len * 2)
        .take_while(|&len| len / 2 < n)
        .fold((sa0, r0), |(_prev_sa, prev_r), len| {
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
            (sa, r)
        })
        .1
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
    let s = read_str();
    let k: usize = read();

    let non_a_all = s.citer().filter(|c| *c != 'a').count();

    let ans = if k >= non_a_all {
        itertools::repeat_n('a', s.len() - k).to_string()
    } else {
        let r = suffix_array_r(&s);

        let (numa, i) = (0..s.len())
            .scan(0, |non_a, i| {
                Some((replace(non_a, *non_a + (s[i] != 'a') as usize), i))
            })
            .take_while(|&(non_a, _)| non_a <= k)
            .map(|(non_a, i)| (k - non_a + i, i))
            .min_by_key(|&(numa, i)| (Reverse(numa), r[i]))
            .unwrap();

        chain(repeat_n('a', numa), s[i..].citer()).to_string()
    };

    println!("{}", ans);
}
