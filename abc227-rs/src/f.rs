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
    let (h, w, k) = read_tuple!(usize, usize, usize);

    let a = read_mat::<usize>(h);

    let b = a
        .iter()
        .flat_map(|row| row.citer())
        .sorted()
        .dedup()
        .collect::<Vec<_>>();
    let idxs = b
        .citer()
        .enumerate()
        .map(|(idx, bb)| (bb, idx))
        .collect::<FxHashMap<_, _>>();

    let m = idxs.len();

    let ans = (0..m)
        .map(|lower| {
            // lowerより大きいのは全部取る, idxと等しいのは取っても取らなくてもokという前提で解を求める
            let mut dp = vec![vec![vec![usize::MAX; k + 1]; w]; h];
            let idx0 = idxs[&a[0][0]];

            if idx0 <= lower {
                dp[0][0][0] = 0;
            }
            if idx0 >= lower {
                dp[0][0][1] = a[0][0];
            }

            for i in 0..h {
                for j in 0..w {
                    for kk in 0..=k {
                        // 右へ
                        if j + 1 < w {
                            let aa = a[i][j + 1];
                            let idx = idxs[&aa];

                            if lower <= idx && kk + 1 <= k {
                                dp[i][j + 1][kk + 1] =
                                    min(dp[i][j + 1][kk + 1], dp[i][j][kk].saturating_add(aa));
                            }

                            if idx <= lower {
                                dp[i][j + 1][kk] = min(dp[i][j + 1][kk], dp[i][j][kk]);
                            }
                        }

                        // 下へ
                        if i + 1 < h {
                            let aa = a[i + 1][j];
                            let idx = idxs[&aa];

                            if lower <= idx && kk + 1 <= k {
                                dp[i + 1][j][kk + 1] =
                                    min(dp[i + 1][j][kk + 1], dp[i][j][kk].saturating_add(aa));
                            }

                            if idx <= lower {
                                dp[i + 1][j][kk] = min(dp[i + 1][j][kk], dp[i][j][kk]);
                            }
                        }
                    }
                }
            }

            // eprintln!("{:?}", dp);

            dp[h - 1][w - 1][k]
        })
        .min()
        .unwrap();

    println!("{}", ans);
}
