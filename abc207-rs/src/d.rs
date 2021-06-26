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

use ordered_float::OrderedFloat;

fn main() {
    let n: usize = read();
    let ab = read_vec(n, || read_tuple!(i64, i64));
    let cd = read_vec(n, || read_tuple!(i64, i64));

    if n == 1 {
        println!("Yes");
        return;
    }

    let ans = (0..n).any(|i| {
        let ab1 = (1..n)
            .map(|j| ab[j])
            .map(|(x, y)| (x - ab[0].0, y - ab[0].1))
            .sorted_by_key(|&(x, y)| (OrderedFloat(f64::atan2(y as f64, x as f64)), x, y))
            .collect::<Vec<_>>();
        let cd1 = (0..n)
            .filter(|&j| j != i)
            .map(|j| cd[j])
            .map(|(x, y)| (x - cd[i].0, y - cd[i].1))
            .collect::<Vec<_>>();
        let t0 = f64::atan2(ab1[0].1 as f64, ab1[0].0 as f64);

        (0..n - 1).any(|j| {
            let t1 = f64::atan2(cd1[j].1 as f64, cd1[j].0 as f64);
            let t = t1 - t0;
            let cd2 = chain(cd1[j..].citer(), cd1[..j].citer())
                .map(|(x, y)| {
                    (
                        t.cos() * x as f64 + t.sin() * y as f64,
                        -t.sin() * x as f64 + t.cos() * y as f64,
                    )
                })
                .collect::<Vec<_>>();
            const EPS: f64 = 1.0e-6;
            if cd2
                .citer()
                .any(|(x, y)| (x - x.round()).abs() > EPS || (y - y.round()).abs() > EPS)
            {
                return false;
            }

            izip!(
                ab1.citer(),
                cd2.citer()
                    .map(|(x, y)| (x.round() as i64, y.round() as i64))
                    .sorted_by_key(|&(x, y)| (OrderedFloat(f64::atan2(y as f64, x as f64)), x, y))
            )
            .all(|(p0, p1)| p0 == p1)
        })
    });

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
