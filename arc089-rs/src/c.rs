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
    let (a, b) = read_tuple!(usize, usize);

    let d = read_mat::<i64>(a);

    let ok = |i: usize, j: usize, kx: usize, ky: usize| {
        if kx * (i + 1) + ky * (j + 1) > d[i][j] as usize {
            return false;
        }

        let kx = kx as i64;
        let ky = ky as i64;

        iproduct!(0..a, 0..b).all(|(ii, jj)| {
            d[i][j] - kx * i as i64 - ky * j as i64 >= d[ii][jj] - kx * ii as i64 - ky * jj as i64
        })
    };

    let k = if let Some(k) = (0..a)
        .map(|i| {
            (0..b)
                .map(|j| iproduct!(0..=100, 0..=100).find(|&(kx, ky)| ok(i, j, kx, ky)))
                .collect::<Option<Vec<_>>>()
        })
        .collect::<Option<Vec<_>>>()
    {
        k
    } else {
        println!("Impossible");
        return;
    };

    println!("Possible");

    let x_vert = |kx: usize| 1 + kx;
    let y_vert = |ky: usize| 202 - ky;

    let edges = k
        .iter()
        .enumerate()
        .flat_map(|(i, k_row)| {
            let d = &d;
            k_row.citer().enumerate().map(move |(j, (kx, ky))| {
                (
                    x_vert(kx),
                    y_vert(ky),
                    d[i][j] - (kx * (i + 1) + ky * (j + 1)) as i64,
                )
            })
        })
        .sorted()
        .dedup()
        .collect::<Vec<_>>();
    println!("{} {}", 202, 200 + edges.len());

    for i in 0..100 {
        println!("{} {} X", x_vert(i), x_vert(i + 1));
    }

    for i in 0..100 {
        println!("{} {} Y", y_vert(i + 1), y_vert(i));
    }

    for (v, u, c) in edges {
        println!("{} {} {}", v, u, c);
    }

    println!("1 202");
}
