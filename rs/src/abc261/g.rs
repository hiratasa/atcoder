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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
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
    let s = read_str();
    let t = read_str();
    let k = read::<usize>();
    let ca = read_vec(k, || {
        let (c, a) = read_tuple!(char, String);
        (c, a.chars().collect::<Vec<_>>())
    });

    let mut costs1 = ca
        .iter()
        .filter(|&(c, a)| a.len() == 1)
        .map(|(c, a)| (*c, a[0]))
        .fold(vec![vec![usize::MAX; 26]; 26], |mut costs1, (c, a)| {
            costs1[c as usize - 'a' as usize][a as usize - 'a' as usize] = 1;
            costs1
        });
    for i in 0..26 {
        costs1[i][i] = 0;
    }
    for k in 0..26 {
        for i in 0..26 {
            for j in 0..26 {
                costs1[i][j] = min(costs1[i][j], costs1[i][k].saturating_add(costs1[k][j]));
            }
        }
    }

    let mapping = ca
        .iter()
        .enumerate()
        .fold(vec![vec![]; 26], |mut mapping, (i_str, (c, _))| {
            mapping[*c as usize - 'a' as usize].push(i_str);
            mapping
        });

    let mut costs_suffix =
        vec![vec![FxHashMap::<(usize, usize), usize>::default(); t.len() + 1]; t.len()];
    let mut costs_char = vec![vec![vec![usize::MAX; 26]; t.len() + 1]; t.len()];
    for l in 1..=t.len() {
        for i in 0..=t.len() - l {
            let dst = &t[i..i + l];

            for i_str in 0..=k {
                let a = if i_str < k { &ca[i_str].1 } else { &s };
                // 最後1文字はskip
                for st in 0..a.len() - 1 {
                    let cost = if &a[st..] == dst {
                        0
                    } else {
                        (i + 1..i + l)
                            .map(|j| {
                                let c1 = if st + 1 == a.len() - 1 {
                                    costs_char[j][i + l][a[a.len() - 1] as usize - 'a' as usize]
                                } else {
                                    *costs_suffix[j][i + l].get(&(i_str, st + 1)).unwrap()
                                };
                                costs_char[i][j][a[st] as usize - 'a' as usize].saturating_add(c1)
                            })
                            .min()
                            .unwrap_or(usize::MAX)
                    };
                    costs_suffix[i][i + l].insert((i_str, st), cost);
                }
            }

            for c in 0..26 {
                costs_char[i][i + l][c] = if l == 1 && t[i] as usize - 'a' as usize == c {
                    0
                } else {
                    mapping[c]
                        .citer()
                        .map(|i_str| {
                            costs_suffix[i][i + l]
                                .get(&(i_str, 0))
                                .copied()
                                .unwrap_or(usize::MAX)
                        })
                        .min()
                        .unwrap_or(usize::MAX)
                        .saturating_add(1)
                };
            }

            for c in 0..26 {
                for c2 in 0..26 {
                    costs_char[i][i + l][c] = min(
                        costs_char[i][i + l][c],
                        costs1[c][c2].saturating_add(costs_char[i][i + l][c2]),
                    );
                }
            }
        }
    }

    let ans = if s.len() == 1 {
        costs_char[0][t.len()][s[0] as usize - 'a' as usize]
    } else {
        costs_suffix[0][t.len()]
            .get(&(k, 0))
            .copied()
            .unwrap_or(usize::MAX)
    };

    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
