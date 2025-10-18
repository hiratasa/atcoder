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
    let n = read::<usize>();
    let xy = read_vec(n, || read_tuple!(i64, i64));

    let dot = |(x0, y0): (i64, i64), (x1, y1): (i64, i64)| x0 * x1 + y0 * y1;

    let cross = |(x0, y0): (i64, i64), (x1, y1): (i64, i64)| x0 * y1 - x1 * y0;

    let (num_over90, num_just90) = (0..n)
        .map(|i| {
            let m = n - 1;

            let t = (0..n)
                .filter(|&j| j != i)
                .map(|j| (xy[j].0 - xy[i].0, xy[j].1 - xy[i].1))
                .sorted_by(|&(x0, y0), &(x1, y1)| {
                    // argsort
                    // https://ngtkana.hatenablog.com/entry/2021/11/13/202103
                    ((y0, x0) < (0, 0))
                        .cmp(&((y1, x1) < (0, 0)))
                        .then_with(|| (x1 * y0).cmp(&(x0 * y1)))
                })
                .collect::<Vec<_>>();

            let num_over90 = (0..m)
                .scan((0, 0), |(k, l), j| {
                    while *l < j + m && cross(t[j], t[*l % m]) >= 0 && dot(t[j], t[*l % m]) >= 0 {
                        *l += 1;
                    }

                    *k = max(*k, *l);
                    while *k < j + m && cross(t[j], t[*k % m]) >= 0 {
                        *k += 1;
                    }

                    Some(*k - *l)
                })
                .sum::<usize>();

            let num_just90 = (0..m)
                .scan(0, |l, j| {
                    while *l < j + m && cross(t[j], t[*l % m]) >= 0 && dot(t[j], t[*l % m]) > 0 {
                        *l += 1;
                    }

                    if cross(t[j], t[*l % m]) >= 0 && dot(t[j], t[*l % m]) == 0 {
                        Some(1)
                    } else {
                        Some(0)
                    }
                })
                .sum::<usize>();

            (num_over90, num_just90)
        })
        .fold((0, 0), |(x, y), (z, w)| (x + z, y + w));

    let num_less90 = n * (n - 1) * (n - 2) / 6 - num_over90 - num_just90;

    println!("{} {} {}", num_less90, num_just90, num_over90);
}
