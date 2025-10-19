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

#[allow(dead_code)]
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

    let t = iproduct!(0..h, 0..w).filter(|&(i, j)| s[i][j] == '#').fold(
        vec![vec![0; h + w]; h + w],
        |mut t, (i, j)| {
            t[i + j][i + (w - j)] = 1;
            t
        },
    );

    let tate = once(vec![0; h + w])
        .chain((0..h + w).scan(vec![0; h + w], |prev, i| {
            izip!(prev.iter_mut(), t[i].citer()).for_each(|(x, y)| *x += y);
            Some(prev.clone())
        }))
        .collect::<Vec<_>>();

    let yoko = (0..h + w)
        .map(|i| {
            once(0)
                .chain(t[i].citer())
                .cumsum::<i64>()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let ans0 = (0..h + w)
        .map(|x| {
            iproduct!(
                (0..)
                    .map(|y| y * 2 + (x + w) % 2)
                    .take_while(|&y| y < h + w),
                (0..)
                    .map(|y| y * 2 + (x + w) % 2)
                    .take_while(|&y| y < h + w)
            )
            .filter(|(y0, y1)| y0 < y1)
            .filter(|&(y0, y1)| t[y0][x] > 0 && t[y1][x] > 0)
            .map(|(y0, y1)| {
                let a = if x >= y1 - y0 {
                    let x0 = x - (y1 - y0);
                    tate[y1 + 1][x0] - tate[y0][x0]
                } else {
                    0
                };
                let b = if x + (y1 - y0) < h + w {
                    let x0 = x + (y1 - y0);
                    tate[y1 + 1][x0] - tate[y0][x0]
                } else {
                    0
                };

                a + b
            })
            .sum::<i64>()
        })
        .sum::<i64>();

    let ans1 = (0..h + w)
        .map(|y| {
            iproduct!(
                (0..)
                    .map(|x| x * 2 + (y + w) % 2)
                    .take_while(|&x| x < h + w),
                (0..)
                    .map(|x| x * 2 + (y + w) % 2)
                    .take_while(|&x| x < h + w)
            )
            .filter(|(x0, x1)| x0 < x1)
            .filter(|&(x0, x1)| t[y][x0] > 0 && t[y][x1] > 0)
            .map(|(x0, x1)| {
                let a = if y >= x1 - x0 {
                    let y0 = y - (x1 - x0);
                    yoko[y0][x1] - yoko[y0][x0 + 1]
                } else {
                    0
                };
                let b = if y + (x1 - x0) < h + w {
                    let y0 = y + (x1 - x0);
                    yoko[y0][x1] - yoko[y0][x0 + 1]
                } else {
                    0
                };

                a + b
            })
            .sum::<i64>()
        })
        .sum::<i64>();
    let ans = ans0 + ans1;
    println!("{}", ans);
}
