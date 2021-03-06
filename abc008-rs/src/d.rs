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

trait Pick0 {
    type Output;
    fn pick0(self) -> Self::Output;
}
impl<T, T2> Pick0 for (T, T2) {
    type Output = T;
    fn pick0(self) -> Self::Output {
        self.0
    }
}
impl<T, T2, T3> Pick0 for (T, T2, T3) {
    type Output = T;
    fn pick0(self) -> Self::Output {
        self.0
    }
}
trait IteratorPick0Ext<T>: std::iter::Iterator<Item = T> + std::marker::Sized
where
    T: Pick0,
{
    fn pick0(self) -> std::iter::Map<Self, fn(T) -> T::Output> {
        self.map(Pick0::pick0)
    }
}
impl<T, I> IteratorPick0Ext<T> for I
where
    I: std::iter::Iterator<Item = T>,
    T: Pick0,
{
}
trait Pick1 {
    type Output;
    fn pick1(self) -> Self::Output;
}
impl<T, T2> Pick1 for (T, T2) {
    type Output = T2;
    fn pick1(self) -> Self::Output {
        self.1
    }
}
impl<T, T2, T3> Pick1 for (T, T2, T3) {
    type Output = T2;
    fn pick1(self) -> Self::Output {
        self.1
    }
}
trait IteratorPick1Ext<T>: std::iter::Iterator<Item = T> + std::marker::Sized
where
    T: Pick1,
{
    fn pick1(self) -> std::iter::Map<Self, fn(T) -> T::Output> {
        self.map(Pick1::pick1)
    }
}
impl<T, I> IteratorPick1Ext<T> for I
where
    I: std::iter::Iterator<Item = T>,
    T: Pick1,
{
}

fn main() {
    let (w, h) = read_tuple!(usize, usize);

    let n: usize = read();
    let xy = read_vec(n, || read_tuple!(usize, usize));

    let xs = once(0)
        .chain(xy.citer().pick0())
        .chain(once(w + 1))
        .sorted()
        .collect_vec();
    let ys = once(0)
        .chain(xy.citer().pick1())
        .chain(once(h + 1))
        .sorted()
        .collect_vec();

    let zz = xy
        .citer()
        .map(|(x, y)| {
            (
                xs.citer().find_position(|&xx| xx == x).unwrap().0,
                ys.citer().find_position(|&yy| yy == y).unwrap().0,
            )
        })
        .collect_vec();

    let ww = xs.len() - 2;
    let hh = ys.len() - 2;

    let dp = iproduct!((1..=ww), (1..=hh)).fold(
        vec![vec![vec![vec![0; hh + 1]; ww + 1]; hh + 1]; ww + 1],
        |dp, (iww, ihh)| {
            iproduct!((0..=ww - iww), (0..=hh - ihh)).fold(dp, |mut dp, (cx, cy)| {
                let dx = cx + iww + 1;
                let dy = cy + ihh + 1;

                let iw = xs[dx] - xs[cx] - 1;
                let ih = ys[dy] - ys[cy] - 1;

                let t = zz
                    .citer()
                    .filter(|&(x, y)| cx < x && x < dx && cy < y && y < dy)
                    // .inspect(|(x, y)| eprintln!("{},{},{},{} : {},{}", iww, ihh, cx, cy, x, y))
                    .map(|(x, y)| {
                        iw + ih - 1
                            + dp[x - cx - 1][y - cy - 1][cx][cy]
                            + dp[dx - x - 1][y - cy - 1][x][cy]
                            + dp[x - cx - 1][dy - y - 1][cx][y]
                            + dp[dx - x - 1][dy - y - 1][x][y]
                    })
                    // .inspect(|tt| eprintln!("{},{},{},{} => {}", iww, ihh, cx, cy, tt))
                    .max()
                    .unwrap_or(0);
                dp[iww][ihh][cx][cy] = t;
                dp
            })
        },
    );
    let ans = dp[ww][hh][0][0];

    println!("{}", ans);
}
