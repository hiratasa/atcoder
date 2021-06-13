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

fn find_smallest_combi(a: &[usize], idxs0: &[usize], idxs1: &[usize], bit_idx: usize) -> usize {
    assert!(!idxs0.is_empty());
    assert!(!idxs1.is_empty());

    if bit_idx == 0 {
        0
    } else {
        let b = bit_idx - 1;
        let (idxs00, idxs01): (Vec<_>, Vec<_>) = idxs0.citer().partition(|&i| (a[i] >> b) & 1 == 0);
        let (idxs10, idxs11): (Vec<_>, Vec<_>) = idxs1.citer().partition(|&i| (a[i] >> b) & 1 == 0);

        let c = iproduct!(
            [&idxs00, &idxs01]
                .citer()
                .enumerate()
                .filter(|&(_, idxs)| !idxs.is_empty()),
            [&idxs10, &idxs11]
                .citer()
                .enumerate()
                .filter(|&(_, idxs)| !idxs.is_empty())
        )
        .map(|((x0, _), (x1, _))| x0 ^ x1)
        .min()
        .unwrap();

        iproduct!(
            [&idxs00, &idxs01]
                .citer()
                .enumerate()
                .filter(|&(_, idxs)| !idxs.is_empty()),
            [&idxs10, &idxs11]
                .citer()
                .enumerate()
                .filter(|&(_, idxs)| !idxs.is_empty())
        )
        .filter(|&((x0, _), (x1, _))| x0 ^ x1 == c)
        .map(|((_, t), (_, u))| find_smallest_combi(&a, t, u, b) | (c << b))
        .min()
        .unwrap()
    }
}

fn solve(a: &[usize], idxs: &[usize], bit_idx: usize) -> usize {
    if bit_idx == 0 {
        0
    } else {
        let b = bit_idx - 1;
        let (idxs0, idxs1): (Vec<_>, Vec<_>) =
            idxs.citer().partition(|&idx| (a[idx] >> b) & 1 == 0);
        if idxs0.is_empty() || idxs1.is_empty() {
            solve(&a, &idxs, b)
        } else if idxs1.len() % 2 == 0 {
            max(solve(&a, &idxs0, b), solve(&a, &idxs1, b))
        } else {
            find_smallest_combi(&a, &idxs0, &idxs1, b) | (1 << b)
        }
    }
}

fn main() {
    let n: usize = read();
    let a = read_row::<usize>();

    let ans = solve(&a, &(0..2 * n).collect::<Vec<_>>(), 30);
    println!("{}", ans);
}
