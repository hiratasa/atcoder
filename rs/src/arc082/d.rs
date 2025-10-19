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
    let x: i64 = read();
    let k: usize = read();
    let r = read_row::<i64>();
    let q: usize = read();
    let ta = read_vec(q, || read_tuple!(i64, i64));

    let simulate = |a: i64| {
        once(0)
            .chain(once(0))
            .chain(r.citer())
            .tuple_windows()
            .map(|(rr0, rr1)| rr1 - rr0)
            .enumerate()
            .scan(a, |aa, (i, delta)| {
                if i % 2 == 0 {
                    *aa = min(x, *aa + delta);
                    Some(*aa)
                } else {
                    *aa = max(0, *aa - delta);
                    Some(*aa)
                }
            })
            .collect::<Vec<_>>()
    };

    let upper = simulate(x);
    let lower = simulate(0);

    let d = once(0)
        .chain(once(0))
        .chain(r.citer())
        .tuple_windows()
        .map(|(rr0, rr1)| rr1 - rr0)
        .enumerate()
        .map(|(i, delta)| if i % 2 == 0 { delta } else { -delta })
        .cumsum::<i64>()
        .collect::<Vec<_>>();
    let d_max = d
        .citer()
        .scan(std::i64::MIN, |m, dd| {
            *m = max(*m, dd);
            Some(*m)
        })
        .collect::<Vec<_>>();
    let d_min = d
        .citer()
        .scan(std::i64::MAX, |m, dd| {
            *m = min(*m, dd);
            Some(*m)
        })
        .collect::<Vec<_>>();

    for (t, a) in ta {
        let idx = r
            .binary_search_by(|&rr| rr.cmp(&t).then(Ordering::Less))
            .unwrap_err();
        let t0 = if idx == 0 { 0 } else { r[idx - 1] };
        let sign = if idx % 2 == 0 { -1 } else { 1 };

        let ans = if a + d_max[idx] >= x {
            // 上にさちってる
            max(0, min(x, upper[idx] + sign * (t - t0)))
        } else if a + d_min[idx] <= 0 {
            // 下にさちってる
            max(0, min(x, lower[idx] + sign * (t - t0)))
        } else {
            max(0, min(x, a + d[idx] + sign * (t - t0)))
        };
        println!("{}", ans);
    }
}
