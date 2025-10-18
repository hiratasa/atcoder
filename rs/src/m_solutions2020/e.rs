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
    let xyp = read_vec(n, || read_tuple!(i64, i64, i64));

    let xidxs = (0..n).sorted_by_key(|&i| xyp[i].0).collect::<Vec<_>>();
    let yidxs = (0..n).sorted_by_key(|&i| xyp[i].1).collect::<Vec<_>>();

    let tx = (0usize..1 << n)
        .map(|s| {
            let bs = bitset!(n, s);

            let v0 = xidxs
                .citer()
                .scan(-(1 << 20), |xmax, idx| {
                    let x = xyp[idx].0;
                    let p = xyp[idx].2;

                    if bs[idx] {
                        *xmax = x;
                    }

                    Some(min((*xmax - x).abs(), x.abs()) * p)
                })
                .collect::<Vec<_>>();

            let v1 = xidxs
                .citer()
                .rev()
                .scan(1 << 20, |xmin, idx| {
                    let x = xyp[idx].0;
                    let p = xyp[idx].2;

                    if bs[idx] {
                        *xmin = x;
                    }

                    Some(min((*xmin - x).abs(), x.abs()) * p)
                })
                .collect::<Vec<_>>();

            izip!(xidxs.citer(), v0, v1.into_iter().rev())
                .map(|(idx, a, b)| (idx, min(a, b)))
                .sorted()
                .map(|(_, a)| a)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let ty = (0usize..1 << n)
        .map(|s| {
            let bs = bitset!(n, s);

            let v0 = yidxs
                .citer()
                .scan(-(1 << 20), |xmax, idx| {
                    let x = xyp[idx].1;
                    let p = xyp[idx].2;

                    if bs[idx] {
                        *xmax = x;
                    }

                    Some(min((*xmax - x).abs(), x.abs()) * p)
                })
                .collect::<Vec<_>>();

            let v1 = yidxs
                .citer()
                .rev()
                .scan(1 << 20, |xmin, idx| {
                    let x = xyp[idx].1;
                    let p = xyp[idx].2;

                    if bs[idx] {
                        *xmin = x;
                    }

                    Some(min((*xmin - x).abs(), x.abs()) * p)
                })
                .collect::<Vec<_>>();

            izip!(yidxs.citer(), v0, v1.into_iter().rev())
                .map(|(idx, a, b)| (idx, min(a, b)))
                .sorted()
                .map(|(_, a)| a)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let ans = (0usize..1 << n)
        .map(|s| {
            let bs = bitset!(n, s);

            let vx = successors(Some(s), |&t| t.checked_sub(1).map(|t| t & s)).fold(
                vec![std::i64::MAX; bs.count_ones() as usize + 1],
                |mut vx, t| {
                    let v = tx[t]
                        .citer()
                        .enumerate()
                        .filter(|&(i, _)| bs[i])
                        .map(|(_, w)| w)
                        .sum::<i64>();

                    let m = t.count_ones() as usize;
                    vx[m] = min(vx[m], v);

                    vx
                },
            );

            let s2 = (!s) & ((1 << n) - 1);
            let vy = successors(Some(s2), |&t| t.checked_sub(1).map(|t| t & s2)).fold(
                vec![std::i64::MAX; (n - bs.count_ones() as usize) + 1],
                |mut vx, t| {
                    let v = ty[t]
                        .citer()
                        .enumerate()
                        .filter(|&(i, _)| !bs[i])
                        .map(|(_, w)| w)
                        .sum::<i64>();

                    let m = t.count_ones() as usize;
                    vx[m] = min(vx[m], v);

                    vx
                },
            );

            iproduct!(vx.citer().enumerate(), vy.citer().enumerate())
                .filter(|&((i, _), (j, _))| i + j <= n)
                .fold(vec![std::i64::MAX; n + 1], |mut t, ((i, v), (j, v2))| {
                    t[i + j] = min(t[i + j], v + v2);
                    t
                })
        })
        .fold(vec![std::i64::MAX; n + 1], |mut ans, t| {
            izip!(ans.iter_mut(), t.citer()).for_each(|(a, b)| {
                *a = min(*a, b);
            });

            ans
        });

    for v in ans {
        println!("{}", v);
    }
}
