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

fn main() {
    let (h, w) = read_tuple!(usize, usize);
    let s = read_vec(h, || read_str());

    let delta0 = [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)];
    let delta1 = iproduct!(
        it!(usize::MAX - 1, usize::MAX, 0, 1, 2),
        it!(usize::MAX - 1, usize::MAX, 0, 1, 2)
    )
    .filter(|&a| {
        it![
            (0, 0),
            (usize::MAX - 1, usize::MAX - 1),
            (usize::MAX - 1, 2),
            (2, usize::MAX - 1),
            (2, 2)
        ]
        .all(|b| a != b)
    })
    .collect::<Vec<_>>();

    let mut costs = vec![vec![usize::MAX; w]; h];
    let mut q = VecDeque::new();

    let chmin = |a: &mut usize, b: usize| {
        if b < *a {
            *a = b;
            true
        } else {
            false
        }
    };

    q.push_back((0, 0, 0));
    while let Some((cost, i, j)) = q.pop_front() {
        if (i, j) == (h - 1, w - 1) {
            println!("{}", cost);
            return;
        }

        if cost > costs[i][j] {
            continue;
        }

        delta0
            .citer()
            .map(|(di, dj)| (i.wrapping_add(di), j.wrapping_add(dj)))
            .filter(|&(ni, nj)| ni < h && nj < w)
            .filter(|&(ni, nj)| s[ni][nj] != '#')
            .filter(|&(ni, nj)| chmin(&mut costs[ni][nj], cost))
            .for_each(|(ni, nj)| {
                q.push_front((cost, ni, nj));
            });

        delta1
            .citer()
            .map(|(di, dj)| (i.wrapping_add(di), j.wrapping_add(dj)))
            .filter(|&(ni, nj)| ni < h && nj < w)
            .filter(|&(ni, nj)| chmin(&mut costs[ni][nj], cost + 1))
            .for_each(|(ni, nj)| {
                q.push_back((cost + 1, ni, nj));
            });
    }
}
