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

// n - m if n > m
fn difference(n: &[u32], m: &[u32]) -> Option<Vec<u32>> {
    if n.len() < m.len() {
        None
    } else {
        let (c, r) = izip!(
            n.citer().rev().chain(repeat(0)),
            m.citer().rev().chain(repeat(0)),
        )
        .take(n.len() + 1)
        .fold((0, vec![]), |(c, r), (d0, d1)| {
            if d0 >= d1 + c {
                (0, pushed!(r, d0 - (d1 + c)))
            } else {
                (1, pushed!(r, 10 + d0 - (d1 + c)))
            }
        });

        if c > 0 {
            None
        } else {
            let r = r
                .into_iter()
                .rev()
                .skip_while(|&d| d == 0)
                .collect::<Vec<_>>();
            if r.is_empty() {
                None
            } else {
                Some(r)
            }
        }
    }
}

fn add(n: &[u32], m: &[u32]) -> Vec<u32> {
    let r = izip!(
        n.citer().rev().chain(repeat(0)),
        m.citer().rev().chain(repeat(0)),
    )
    .take(n.len() + 1)
    .fold((0, vec![]), |(c, r), (d0, d1)| {
        if d0 + d1 + c >= 10 {
            (1, pushed!(r, d0 + d1 + c - 10))
        } else {
            (0, pushed!(r, d0 + d1 + c))
        }
    })
    .1;

    r.into_iter()
        .rev()
        .skip_while(|&d| d == 0)
        .collect::<Vec<_>>()
}

fn main() {
    let n = read_str()
        .into_iter()
        .map(|d| d.to_digit(10).unwrap())
        .collect_vec();
    let l = n.len();

    let idx0 = n.citer().position(|d| d != 0).unwrap_or(l);
    let (n0, next) = if idx0 == 0 {
        let idx = 1 + n[1..].citer().position(|d| d != 0).unwrap_or(l - 1);
        (n[0..idx].to_vec(), idx)
    } else {
        (once(1).chain(n[0..idx0].citer()).collect::<Vec<_>>(), idx0)
    };

    if next == l {
        println!("{} {}", n0.citer().join(""), 1);
        return;
    }

    let diff = (1..)
        .find_map(|len| {
            if next + len > l {
                if len < n0.len() {
                    None
                } else if len == n0.len() {
                    if n0.starts_with(&n[next..]) {
                        if n0.citer().skip(l - next).all(|d| d == 9) {
                            None
                        } else {
                            Some(vec![1])
                        }
                    } else if &n0[0..l - next] < &n[next..] {
                        let n1 = n[next..].citer().chain(repeat(0)).take(len).collect_vec();
                        difference(&n1, &n0)
                    } else {
                        None
                    }
                } else {
                    let n1 = n[next..].citer().chain(repeat(0)).take(len).collect_vec();
                    let diff = difference(&n1, &n0);
                    assert!(diff.is_some());
                    diff
                }
            } else {
                let n1 = &n[next..next + len];
                let diff = difference(n1, &n0)?;

                let n1 = n1.citer().collect::<Vec<_>>();
                let ok = iterate(n1, |m| add(m, &diff))
                    .scan(next, |idx, m| Some((replace(idx, *idx + m.len()), m)))
                    .take_while(|t| t.0 < l)
                    .all(|(idx, m)| {
                        if n[idx] == 0 {
                            false
                        } else if idx + m.len() <= l {
                            m == &n[idx..idx + m.len()]
                        } else {
                            m.starts_with(&n[idx..])
                        }
                    });

                if ok {
                    Some(diff)
                } else {
                    None
                }
            }
        })
        .unwrap();
    println!("{} {}", n0.citer().join(""), diff.citer().join(""));
}
