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
    let n: usize = read();

    let xy = read_vec(n, || read_tuple!(i64, i64));

    let dir = |x: i64, y: i64| {
        if y == 0 {
            if x > 0 { 0 } else { 2 }
        } else if y > 0 {
            1
        } else {
            3
        }
    };

    let xy = xy
        .citer()
        .filter(|&(x, y)| (x, y) != (0, 0))
        .sorted_by(|&(x0, y0), &(x1, y1)| {
            if dir(x0, y0) != dir(x1, y1) {
                dir(x0, y0).cmp(&dir(x1, y1))
            } else {
                (x0 * y1 - x1 * y0).cmp(&0).reverse()
            }
        })
        .coalesce(|(x0, y0), (x1, y1)| {
            if x0 * y1 - x1 * y0 == 0 && dir(x0, y0) == dir(x1, y1) {
                Ok((x0 + x1, y0 + y1))
            } else {
                Err(((x0, y0), (x1, y1)))
            }
        })
        .collect::<Vec<_>>();

    let m = xy.len();

    let ans = (0..m)
        .scan((0, 0, 0), |(j, xx, yy), i| {
            let (x0, y0) = xy[i];

            let t = (*j..i + m)
                .take_while(|&jj| {
                    let (x1, y1) = xy[jj % m];

                    jj == i || x0 * y1 - x1 * y0 > 0
                })
                .fold((*j, *xx, *yy, 0), |(_, xxx, yyy, d), jj| {
                    let (x1, y1) = xy[jj % m];

                    (
                        jj + 1,
                        xxx + x1,
                        yyy + y1,
                        max(d, (xxx + x1).pow(2) + (yyy + y1).pow(2)),
                    )
                });

            *j = t.0;
            *xx = t.1;
            *yy = t.2;

            let d0 = t.3;

            *xx -= x0;
            *yy -= y0;

            let d1 = *xx * *xx + *yy * *yy;

            let t = (*j..i + m)
                .take_while(|&jj| {
                    let (x1, y1) = xy[jj % m];

                    x0 * y1 - x1 * y0 == 0
                })
                .fold((*j, *xx, *yy), |(_, xxx, yyy), jj| {
                    let (x1, y1) = xy[jj % m];

                    (jj + 1, xxx + x1, yyy + y1)
                });

            *j = t.0;
            *xx = t.1;
            *yy = t.2;

            let d2 = *xx * *xx + *yy * *yy;

            Some(max(d0, max(d1, d2)))
        })
        .max()
        .map(|d2| (d2 as f64).sqrt())
        .unwrap_or(0.0);
    println!("{}", ans);
}
