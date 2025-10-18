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

#[allow(dead_code)]
fn solve0(n: usize, k: usize, a: &[usize]) -> usize {
    let idxs = a
        .citer()
        .enumerate()
        .fold(vec![vec![]; k], |mut idxs, (i, x)| {
            idxs[x - 1].push(i);
            idxs
        });

    idxs.iter()
        .multi_cartesian_product()
        .map(|vs: Vec<&usize>| {
            let remains = (0..n).filter(|&i| !vs.contains(&&i)).collect::<Vec<_>>();

            (0..=remains.len())
                .map(|i| {
                    let seq = remains[..i]
                        .citer()
                        .chain(vs.iter().copied().copied())
                        .chain(remains[i..].citer())
                        .collect::<Vec<_>>();

                    assert!(seq.len() == n);

                    (0..n)
                        .map(|j| seq[0..j].citer().filter(|&idx| idx > seq[j]).count())
                        .sum::<usize>()
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn main() {
    let (n, k) = read_tuple!(usize, usize);
    let a = read_row::<usize>();

    let dpl = once(vvec![0; usize::MAX; 1<<k])
        .chain(a.citer().scan(vvec![0; usize::MAX; 1<<k], |dp, x| {
            let x = x - 1;

            let mut next = vec![usize::MAX; 1 << k];

            for s in 0..1 << k {
                // xを使わない
                next[s] = min(next[s], dp[s].saturating_add(s.count_ones() as usize));

                // xを使う
                if s & (1 << x) == 0 {
                    let cost = (s & (!0 - ((1 << x) - 1))).count_ones() as usize;
                    next[s ^ (1 << x)] = min(next[s ^ (1 << x)], dp[s].saturating_add(cost));
                }
            }

            *dp = next;

            Some(dp.clone())
        }))
        .collect::<Vec<_>>();

    let dpr = once(vvec![0; usize::MAX; 1<<k])
        .chain(a.citer().rev().scan(vvec![0; usize::MAX; 1<<k], |dp, x| {
            let x = x - 1;

            let mut next = vec![usize::MAX; 1 << k];

            for s in 0..1 << k {
                // xを使わない
                next[s] = min(next[s], dp[s].saturating_add(s.count_ones() as usize));

                // xを使う
                if s & (1 << x) == 0 {
                    let cost = (s & ((1 << x) - 1)).count_ones() as usize;
                    next[s ^ (1 << x)] = min(next[s ^ (1 << x)], dp[s].saturating_add(cost));
                }
            }

            *dp = next;

            Some(dp.clone())
        }))
        .collect::<Vec<_>>();

    let ans = (0usize..1 << k)
        .filter_map(|s| {
            let t = (1 << k) - 1 - s;
            let add = (0..k)
                .filter(|&i| s & (1 << i) > 0)
                .map(|i| (t & ((1 << i) - 1)).count_ones() as usize)
                .sum::<usize>();

            (0..=n)
                .map(|i| dpl[i][s].saturating_add(dpr[n - i][t]).saturating_add(add))
                .min()
        })
        .min()
        .unwrap();

    println!("{}", ans);
}
