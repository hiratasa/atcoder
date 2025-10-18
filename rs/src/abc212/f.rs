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
        if $n > 0 {
            bs.buffer_mut()[0] = $x as u64;
        }
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
    let (n, m, q) = read_tuple!(usize, usize, usize);
    let trains = read_vec(m, || read_tuple!(usize, usize, usize, usize));
    let query = read_vec(q, || read_tuple!(usize, usize, usize));

    let mut timetable = trains.citer().sorted_by_key(|&(_, _, s, _)| s).fold(
        vec![vec![]; n],
        |mut timetable, (a, b, s, t)| {
            timetable[a - 1].push((b - 1, s, t));
            timetable
        },
    );
    // sentinel
    for i in 0..n {
        timetable[i].push((i, usize::MAX, usize::MAX));
    }

    let mut nexts = timetable
        .iter()
        .map(|row| {
            row.citer()
                .map(|(b, _s, t)| {
                    let idx = timetable[b]
                        .binary_search_by(|&(_, ss, _)| ss.cmp(&t).then(Ordering::Greater))
                        .unwrap_err();

                    vec![(b, idx)]
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for i in 0..20 {
        for j in 0..n {
            for k in 0..timetable[j].len() {
                let z = nexts[nexts[j][k][i].0][nexts[j][k][i].1][i];
                nexts[j][k].push(z);
            }
        }
    }

    query
        .citer()
        .map(|(x, y, z)| {
            let y = y - 1;

            let start = timetable[y]
                .binary_search_by(|&(_, s, _)| s.cmp(&x).then(Ordering::Greater))
                .unwrap_err();

            if z <= timetable[y][start].1 {
                Err(y)
            } else {
                let (q, idx) = (0..20).rev().fold((y, start), |(p, t), i| {
                    let (q, idx) = nexts[p][t][i];

                    if z <= timetable[q][idx].1 {
                        (p, t)
                    } else {
                        (q, idx)
                    }
                });

                if z <= timetable[q][idx].2 {
                    Ok((q, timetable[q][idx].0))
                } else {
                    Err(timetable[q][idx].0)
                }
            }
        })
        .for_each(|ans| match ans {
            Ok((a, b)) => {
                println!("{} {}", a + 1, b + 1);
            }
            Err(a) => {
                println!("{}", a + 1);
            }
        });
}
