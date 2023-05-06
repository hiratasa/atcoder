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
    let x: i64 = read();
    let k: usize = read();
    let r = read_row::<i64>();

    let r = once(0)
        .chain(r.citer())
        .chain(once(std::i64::MAX))
        .collect_vec();

    let q: usize = read();
    let query = read_vec(q, || read_tuple!(i64, i64));

    let c = once(0)
        .chain(r.citer().scan(0, |p, rr| {
            let t = rr - *p;
            *p = t;
            Some(*p)
        }))
        .collect_vec();

    let limit = (0..=k)
        .map(|i| {
            if i % 2 == 0 {
                c[i + 2] - c[i + 1]
            } else {
                x - c[i + 2] + c[i + 1]
            }
        })
        .enumerate()
        .fold(vec![0, x], |limit, (i, aa)| {
            let t = if i % 2 == 0 {
                max(aa, limit[limit.len() - 2])
            } else {
                min(aa, limit[limit.len() - 2])
            };

            pushed!(limit, t)
        });

    let b0 = (0..=k)
        .scan(0, |p, i| {
            let p0 = *p;
            if i % 2 == 0 {
                *p = max(*p - (r[i + 1] - r[i]), 0);
            } else {
                *p = min(*p + (r[i + 1] - r[i]), x);
            }

            Some(p0)
        })
        .collect_vec();

    let bx = (0..=k)
        .scan(x, |p, i| {
            let p0 = *p;
            if i % 2 == 0 {
                *p = max(*p - (r[i + 1] - r[i]), 0);
            } else {
                *p = min(*p + (r[i + 1] - r[i]), x);
            }

            Some(p0)
        })
        .collect_vec();
    for (t, a) in query {
        let idx = r
            .binary_search_by(|&rr| rr.cmp(&t).then(Ordering::Less))
            .unwrap_err()
            - 1;

        let ans = if idx % 2 == 0 {
            let st = if limit[idx] >= a {
                // 下に張り付いてる
                b0[idx]
            } else if limit[idx + 1] <= a {
                // 上に張り付いてる
                bx[idx]
            } else {
                a + c[idx + 1] - c[idx]
            };

            max(0, st - (t - r[idx]))
        } else {
            let st = if limit[idx + 1] >= a {
                // 下に張り付いてる
                b0[idx]
            } else if limit[idx] <= a {
                // 上に張り付いてる
                bx[idx]
            } else {
                a + c[idx] - c[idx + 1]
            };

            min(x, st + (t - r[idx]))
        };

        println!("{}", ans);
    }
}
