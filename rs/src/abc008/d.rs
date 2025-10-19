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
    let (w, h) = read_tuple!(usize, usize);
    let n = read::<usize>();
    let xy = read_vec(n, || read_tuple!(usize, usize));

    let xs = it![0, w + 1]
        .chain(xy.citer().map(|(x, _)| x))
        .sorted()
        .dedup()
        .collect::<Vec<_>>();
    let ys = it![0, h + 1]
        .chain(xy.citer().map(|(_, y)| y))
        .sorted()
        .dedup()
        .collect::<Vec<_>>();

    let nx = xs.len();
    let ny = ys.len();

    let dp = iproduct!(1..nx, 0..nx, 1..ny, 0..ny)
        .filter(|&(ww, i, hh, j)| i + ww < nx && j + hh < ny)
        .fold(
            vec![vec![vec![vec![0; ny]; ny]; nx]; nx],
            |mut dp, (ww, i0, hh, j0)| {
                let i1 = i0 + ww;
                let j1 = j0 + hh;

                let x0 = xs[i0];
                let x1 = xs[i1];
                let y0 = ys[j0];
                let y1 = ys[j1];

                let z = xy
                    .citer()
                    .filter(|&(x, y)| x0 < x && x < x1 && y0 < y && y < y1)
                    .map(|(x, y)| {
                        let i2 = xs.binary_search(&x).unwrap();
                        let j2 = ys.binary_search(&y).unwrap();

                        dp[i0][i2][j0][j2]
                            + dp[i0][i2][j2][j1]
                            + dp[i2][i1][j0][j2]
                            + dp[i2][i1][j2][j1]
                            + (y1 - y0 - 1)
                            + (x1 - x0 - 1)
                            - 1
                    })
                    .max()
                    .unwrap_or(0);

                dp[i0][i1][j0][j1] = z;

                dp
            },
        );

    let ans = dp[0][nx - 1][0][ny - 1];

    println!("{}", ans);
}
