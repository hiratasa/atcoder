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

fn main() {
    let (h, w, t) = read_tuple!(usize, usize, usize);
    let a = read_vec(h, || read_str());

    let start = a
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.citer().position(|c| c == 'S').map(|j| (i, j)))
        .unwrap();
    let goal = a
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.citer().position(|c| c == 'G').map(|j| (i, j)))
        .unwrap();
    let candies = a
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.citer().positions(|c| c == 'o').map(move |j| (i, j)))
        .collect::<Vec<_>>();

    let n = candies.len();

    let calc_dists = |(i0, j0): (usize, usize)| {
        let mut q = VecDeque::new();
        let mut dists = vec![vec![usize::MAX; w]; h];

        q.push_back((0, (i0, j0)));
        dists[i0][j0] = 0;

        while let Some((d, (i, j))) = q.pop_front() {
            if d >= t {
                continue;
            }

            q.extend(
                it![(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)]
                    .map(|(ni, nj)| (i.wrapping_add(ni), j.wrapping_add(nj)))
                    .filter(|&(ni, nj)| ni < h && nj < w)
                    .filter(|&(ni, nj)| a[ni][nj] != '#')
                    .filter(|&(ni, nj)| {
                        if d + 1 < dists[ni][nj] {
                            dists[ni][nj] = d + 1;
                            true
                        } else {
                            false
                        }
                    })
                    .map(|(ni, nj)| (d + 1, (ni, nj))),
            );
        }

        dists
    };

    let targets = candies
        .citer()
        .chain(once(goal))
        .chain(once(start))
        .collect::<Vec<_>>();

    let dists_table = targets
        .citer()
        .map(|coord0| calc_dists(coord0))
        .collect::<Vec<_>>();

    let mut dp = vec![vec![usize::MAX; 1 << (n + 1)]; n + 2];
    dp[n + 1][0] = 0;

    let dp = iproduct!(0..1 << n, 0..n + 2)
        .filter(|&(s, idx)| s & (1 << idx) > 0 || idx == n + 1)
        .fold(dp, |mut dp, (s, idx)| {
            let d0 = dp[idx][s];
            if d0 == usize::MAX {
                return dp;
            }

            (0..=n)
                .filter(|&idx2| s & (1 << idx2) == 0)
                .for_each(|idx2| {
                    let d = d0.saturating_add(dists_table[idx][targets[idx2].0][targets[idx2].1]);

                    if d < dp[idx2][s ^ (1 << idx2)] {
                        dp[idx2][s ^ (1 << idx2)] = d;
                    }
                });

            dp
        });

    let ans = (1 << n..1 << (n + 1))
        .filter(|&s| dp[n][s] <= t)
        .map(|s| s.count_ones() - 1)
        .max();

    println_opt(ans);
}
