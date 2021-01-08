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

fn calc(x: i64, y: i64, n: i64) -> Box<dyn Iterator<Item = (i64, i64)>> {
    if n == 1 {
        Box::new(it!((x, y), (x + 1, y), (x + 1, y + 1)))
    } else {
        // length = 3^n - (3^n - 1) / 2
        let len_n_1 = 3i64.pow(n as u32 - 1) - (3i64.pow(n as u32 - 1)) / 2;
        Box::new(
            calc(x, y, n - 1)
                .chain(calc(x + 2 * len_n_1 - 1, y, n - 1))
                .chain(calc(x + 2 * len_n_1 - 1, y + 2 * len_n_1 - 1, n - 1)),
        )
    }
}

fn verify(xy: &Vec<(i64, i64)>) -> (usize, usize, usize, usize) {
    assert!(0 < xy.len());
    assert!(xy.len() <= 100000);
    xy.citer().for_each(|(x, y)| {
        assert!(0 <= x);
        assert!(x <= 1000000000);
        assert!(0 <= y);
        assert!(y <= 1000000000);
    });
    assert!(xy.len() == xy.citer().unique().count());

    let a = xy.citer().map(|(x, y)| y).unique().count();
    let b = xy.citer().map(|(x, y)| x - y).unique().count();
    let c = xy.citer().map(|(x, y)| x).unique().count();
    let d = xy.citer().map(|(x, y)| x + y).unique().count();

    (a, b, c, d)
}

fn main() {
    let ans = calc(0, 0, 6).collect_vec();

    // println!("{:?}", verify(&ans));
    println!("{}", ans.len());
    for (x, y) in ans {
        println!("{} {}", x, y);
    }
}
