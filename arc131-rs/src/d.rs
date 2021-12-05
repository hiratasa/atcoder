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
    let (n, m, d) = read_tuple!(usize, usize, i64);
    let r = read_row::<i64>();
    let s = read_row::<i64>();

    let ans = if n % 2 == 0 {
        let lower0 = -((n / 2) as i64) * d;
        let upper0 = ((n / 2) as i64 - 1) * d;

        let mut vals = izip!(r.citer().tuple_windows(), s.citer())
            .map(|((r0, r1), ss)| {
                (((max(lower0, -r0) - 1).div_euclid(d) - (max(lower0, -r1) - 1).div_euclid(d))
                    + ((min(upper0, r1)).div_euclid(d) - (min(upper0, r0)).div_euclid(d)))
                    * ss
            })
            .collect::<Vec<_>>();
        let p0 = vals.citer().sum::<i64>();

        let events = r.citer().tuple_windows().enumerate().fold(
            vec![vec![]; d as usize],
            |mut events, (i, (r0, r1))| {
                events[(-r1).rem_euclid(d) as usize].push(i);
                events[(-r0).rem_euclid(d) as usize].push(i);
                events[(r0 + 1).rem_euclid(d) as usize].push(i);
                events[(r1 + 1).rem_euclid(d) as usize].push(i);

                events
            },
        );

        (0..d)
            .map(|i| (i as usize, lower0 + i, upper0 + i))
            .scan(p0, |p, (i, lower, upper)| {
                let b = if i == 0 { s[0] } else { 0 };

                let ii = i as i64;
                for &idx in &events[i] {
                    let r0 = r[idx];
                    let r1 = r[idx + 1];

                    // [max(lower, -r1), max(lower, -r0)), (min(upper, r0), min(upper, r1)]
                    let plus = (((max(lower, -r0) - ii - 1).div_euclid(d)
                        - (max(lower, -r1) - ii - 1).div_euclid(d))
                        + ((min(upper, r1) - ii).div_euclid(d)
                            - (min(upper, r0) - ii).div_euclid(d)))
                        * s[idx];

                    *p -= vals[idx];
                    *p += plus;
                    vals[idx] = plus;
                }

                Some(*p + b)
            })
            .max()
            .unwrap()
    } else {
        let lower0 = -((n / 2) as i64) * d;
        let upper0 = ((n / 2) as i64) * d;

        let mut vals = izip!(r.citer().tuple_windows(), s.citer())
            .map(|((r0, r1), ss)| (min(r1, upper0) / d - min(r0, upper0) / d) * 2 * ss)
            .collect::<Vec<_>>();
        let p0 = vals.citer().sum::<i64>();

        let events = r.citer().tuple_windows().enumerate().fold(
            vec![vec![]; d as usize],
            |mut events, (i, (r0, r1))| {
                events[(-r1).rem_euclid(d) as usize].push(i);
                events[(-r0).rem_euclid(d) as usize].push(i);
                events[(r0 + 1).rem_euclid(d) as usize].push(i);
                events[(r1 + 1).rem_euclid(d) as usize].push(i);

                events
            },
        );

        (0..=d / 2)
            .map(|i| {
                (
                    i as usize,
                    -((n / 2) as i64) * d + (i as i64),
                    (n / 2) as i64 * d + (i as i64),
                )
            })
            .scan(p0, |p, (i, lower, upper)| {
                let b = if i == 0 { s[0] } else { 0 };

                let ii = i as i64;
                for &idx in &events[i] {
                    let r0 = r[idx];
                    let r1 = r[idx + 1];

                    // [max(lower, -r1), max(lower, -r0)), (min(upper, r0), min(upper, r1)]
                    let plus = (((max(lower, -r0) - ii - 1).div_euclid(d)
                        - (max(lower, -r1) - ii - 1).div_euclid(d))
                        + ((min(upper, r1) - ii).div_euclid(d)
                            - (min(upper, r0) - ii).div_euclid(d)))
                        * s[idx];

                    *p -= vals[idx];
                    *p += plus;
                    vals[idx] = plus;
                }

                Some(*p + b)
            })
            .max()
            .unwrap()
    };

    println!("{}", ans);
}
