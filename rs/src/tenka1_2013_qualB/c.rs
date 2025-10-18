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
fn println_opt<T: Copy + std::fmt::Display>(ans: Option<T>) {
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
    let (n, m, a, b) = read_tuple!(usize, usize, usize, usize);

    let mut init =
        vec![vec![vec![vec![None; (n - 2) * (m - 2) + 1]; (n - 2) * (m - 2) + 1]; 2]; 1 << m];
    init[0][0][0][0] = Some(0);
    let dp = iproduct!(0..n, 0..m).fold(init, |prev, (i, j)| {
        let mut next =
            vec![vec![vec![vec![None; (n - 2) * (m - 2) + 1]; (n - 2) * (m - 2) + 1]; 2]; 1 << m];
        for s in 0..1 << m {
            for c in 0..2 {
                for x in 0..=(n - 2) * (m - 2) {
                    for y in 0..=(n - 2) * (m - 2) {
                        if prev[s][c][x][y].is_none() {
                            continue;
                        }

                        let top = if i == 0 { 2 } else { 1 - (s & 1) };
                        let left = if j == 0 { 2 } else { 1 - c };

                        let rights = if j == m - 1 {
                            [2].as_ref()
                        } else {
                            [0, 1].as_ref()
                        };
                        for &right in rights {
                            let bottoms = if i == n - 1 {
                                [2].as_ref()
                            } else {
                                [0, 1].as_ref()
                            };
                            for &bottom in bottoms {
                                let c0 = [top, left, right, bottom]
                                    .citer()
                                    .filter(|&x| x == 0)
                                    .count();
                                let c1 = [top, left, right, bottom]
                                    .citer()
                                    .filter(|&x| x == 1)
                                    .count();
                                let c2 = [top, left, right, bottom]
                                    .citer()
                                    .filter(|&x| x == 2)
                                    .count();

                                let next_s = (s >> 1) | ((bottom % 2) << (m - 1));
                                let next_c = right % 2;

                                let next_x = x + (c0 == 4) as usize;
                                let next_y = y + (c1 == 4) as usize;

                                let add_val = (c2 == 1 && c0 == 3) as usize;
                                let next_val = prev[s][c][x][y].map(|val| val + add_val);

                                next[next_s][next_c][next_x][next_y] =
                                    max(next[next_s][next_c][next_x][next_y], next_val);
                            }
                        }
                    }
                }
            }
        }

        next
    });

    let ans = iproduct!(0..1 << m, 0..2, a..=(n - 2) * (m - 2), 0..=b)
        .map(|(s, c, x, y)| dp[s][c][x][y].unwrap_or(0))
        .max()
        .unwrap();

    println!("{}", ans);
}
