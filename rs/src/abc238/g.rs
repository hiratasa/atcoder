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
    let (n, q) = read_tuple!(usize, usize);
    let a = read_row::<usize>();
    let lr = read_vec(q, || read_tuple!(usize, usize));

    const M: usize = 1000000;

    let (t, num_primes) = (2..=M).fold(
        (vec![None; M + 1], 0),
        |(mut t, idx): (Vec<Option<(usize, usize)>>, usize), i| {
            if t[i].is_none() {
                t[i] = Some((i, idx));
                (
                    (2..)
                        .map(|j| i * j)
                        .take_while(|&j| j <= M)
                        .fold(t, |mut t, j| {
                            t[j] = t[j].or(Some((i, idx)));
                            t
                        }),
                    idx + 1,
                )
            } else {
                (t, idx)
            }
        },
    );

    let prime_factors = a
        .citer()
        .map(|aa| {
            successors(Some(aa), |&b| t[b].map(|(p, _)| b / p))
                .filter_map(|x| t[x])
                .sorted()
                .group_by(|&p| p)
                .into_iter()
                .map(|((_p, idx), it)| (idx, it.count()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let add = |v: &mut [usize], s: &mut usize, i: usize| {
        prime_factors[i].citer().for_each(|(idx, m)| {
            let old = v[idx];
            v[idx] += m;
            v[idx] %= 3;
            let new = v[idx];
            *s = *s - old + new;
        });
    };
    let remove = |v: &mut [usize], s: &mut usize, i: usize| {
        prime_factors[i].citer().for_each(|(idx, m)| {
            let old = v[idx];
            v[idx] += 3 - m % 3;
            v[idx] %= 3;
            let new = v[idx];
            *s = *s - old + new;
        });
    };

    let b = (n as f64).sqrt() as usize;
    let ans = lr
        .citer()
        .map(|(l, r)| (l - 1, r))
        .enumerate()
        .sorted_by_key(|&(_, (l, r))| (l / b, r))
        .fold(
            (vec![false; q], vec![0; num_primes], 0, 0, 0),
            |(mut ans, mut v, mut s, mut l0, mut r0), (i_query, (l, r))| {
                while l < l0 {
                    add(&mut v, &mut s, l0 - 1);
                    l0 -= 1;
                }
                while r0 < r {
                    add(&mut v, &mut s, r0);
                    r0 += 1;
                }
                while l0 < l {
                    remove(&mut v, &mut s, l0);
                    l0 += 1;
                }
                while r < r0 {
                    remove(&mut v, &mut s, r0 - 1);
                    r0 -= 1;
                }

                ans[i_query] = s == 0;

                (ans, v, s, l0, r0)
            },
        )
        .0;

    for a in ans {
        if a {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
