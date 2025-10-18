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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom};
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
    let (n, k) = read_tuple!(usize, usize);
    let a = read_row::<usize>();

    let left = a
        .citer()
        .scan(vec![0], |t, x| {
            let idx = t
                .binary_search_by(|&y| y.cmp(&x).then(Ordering::Greater))
                .unwrap_err();

            if idx == t.len() {
                t.push(x);
            } else {
                t[idx] = x;
            }

            Some(idx)
        })
        .collect::<Vec<_>>();

    let right = a
        .citer()
        .rev()
        .map(|x| x.wrapping_neg())
        .scan(vec![0], |t, x| {
            let idx = t
                .binary_search_by(|&y| y.cmp(&x).then(Ordering::Greater))
                .unwrap_err();

            if idx == t.len() {
                t.push(x);
            } else {
                t[idx] = x;
            }

            Some(idx)
        })
        .collect::<Vec<_>>();

    let l = left.citer().max().unwrap();

    let t = izip!(a, left, right.into_iter().rev())
        .filter(|&(_x, l0, l1)| l0 + l1 == l + 1)
        .map(|(x, _, l1)| (x, l1))
        .rev()
        .fold(
            vvec![vec![(0, 0usize, 0usize, 0usize, 0usize), (usize::MAX, 1, 0, 0, 0)]; vec![(0, 0, 0, 0, 0)]; l + 1],
            |mut t, (x, l1)| {
                assert!(t[l1][t[l1].len() - 1].0 <= x);

                let prev = t[l1][t[l1].len() - 1].2;
                let (idx0, idx1) = if t[l1][t[l1].len() - 1].0 == x {
                    (
                        prev,
                        t[l1][t[l1].len() - 1].4
                    )
                } else {
                    let idx = prev + t[l1 - 1][prev..].citer().position(|(y, _, _, _, _)| y > x).unwrap();

                    (idx, idx)
                };

                let c = t[l1 - 1][t[l1 - 1].len() - 1].1 - t[l1 - 1][idx1 - 1].1;

                let c0 = t[l1][t[l1].len() - 1].1;
                let idx2 = t[l1 - 1].len();
                t[l1].push((x, c0.saturating_add(c), idx0, idx1, idx2));

                t
            },
        );

    if t[l][t[l].len() - 1].1 < k {
        println!("None");
        return;
    }

    let ans = (0..l)
        .scan((1, k), |(idx0, kk), i| {
            let idx1 = *idx0
                + t[l - i][*idx0..]
                    .citer()
                    .position(|(_, c, _, _, _)| c - t[l - i][*idx0 - 1].1 >= *kk)
                    .unwrap();

            *kk -= t[l - i][idx1 - 1].1 - t[l - i][*idx0 - 1].1;
            *idx0 = t[l - i][idx1].3;

            Some(t[l - i][idx1].0)
        })
        .collect::<Vec<_>>();

    println!("{}", ans.citer().format(" "));
}
