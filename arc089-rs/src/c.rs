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
    let d = read_mat::<usize>(a);

    let t = iproduct!(0..a, 0..b)
        .try_fold(vec![vec![(0, 0); b]; a], |mut t, (i, j)| {
            let (nx, ny) = (0usize..=100)
                .filter_map(|nx| {
                    iproduct!(0..a, 0..b)
                        .try_fold((0i64, 100i64), |(l, u), (ii, jj)| {
                            let dx = ii as i64 - i as i64;
                            let dy = jj as i64 - j as i64;
                            let dd = d[ii][jj] as i64 - d[i][j] as i64;

                            // nx * dx + ny * dy >= dd
                            if dy > 0 {
                                Some((max(l, (dd - nx as i64 * dx + dy - 1).div_euclid(dy)), u))
                            } else if dy < 0 {
                                Some((l, min(u, (nx as i64 * dx - dd).div_euclid(-dy))))
                            } else if nx as i64 * dx < dd {
                                None
                            } else {
                                Some((l, u))
                            }
                        })
                        .filter(|&(l, u)| l <= u)
                        .map(|(l, _u)| (nx, l as usize))
                })
                .min_by_key(|&(nx, ny)| nx * (i + 1) + ny * (j + 1))?;

            t[i][j] = (nx, ny);

            Some(t)
        })
        .unwrap_or_else(|| {
            println!("Impossible");
            std::process::exit(0)
        });

    let c = iproduct!(0..a, 0..b)
        .try_fold(vec![vec![0; b]; a], |mut c, (i, j)| {
            c[i][j] = d[i][j].checked_sub((i + 1) * t[i][j].0 + (j + 1) * t[i][j].1)?;
            Some(c)
        })
        .unwrap_or_else(|| {
            println!("Impossible");
            std::process::exit(0)
        });

    eprintln!("{:?}", d);
    eprintln!("{:?}", c);

    println!("Possible");
    let edges = iproduct!(0..a, 0..b)
        .map(|(i, j)| (t[i][j], c[i][j]))
        .sorted()
        .dedup()
        .collect::<Vec<_>>();

    println!("{} {}", 201, 200 + edges.len());
    for i in 0..100 {
        println!("{} {} X", i + 1, i + 2);
    }
    for i in 0..100 {
        println!("{} {} Y", i + 101, i + 102);
    }

    for ((i, j), w) in edges {
        println!("{} {} {}", i + 1, 201 - j, w);
    }

    println!("{} {}", 1, 201);
}
