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

use bitset_fixed::BitSet;

fn main() {
    let b = read_mat::<usize>(2);
    let c = read_mat::<usize>(3);

    let total = b
        .iter()
        .map(|row| row.citer().sum::<usize>())
        .sum::<usize>()
        + c.iter()
            .map(|row| row.citer().sum::<usize>())
            .sum::<usize>();

    let points = (0..1 << 9)
        .filter(|&s: &u64| s.count_ones() == 4)
        .map(|s| {
            let mut bs = BitSet::new(9);
            bs.buffer_mut()[0] = s;

            let pb = b
                .iter()
                .enumerate()
                .map(|(i, brow)| {
                    brow.citer()
                        .enumerate()
                        .filter(|(j, _)| bs[3 * i + j] == bs[3 * (i + 1) + j])
                        .map(|(_, bb)| bb)
                        .sum::<usize>()
                })
                .sum::<usize>();
            let pc = c
                .iter()
                .enumerate()
                .map(|(i, brow)| {
                    brow.citer()
                        .enumerate()
                        .filter(|(j, _)| bs[3 * i + j] == bs[3 * i + j + 1])
                        .map(|(_, cc)| cc)
                        .sum::<usize>()
                })
                .sum::<usize>();
            (((1 << 9) - 1, s), (pb + pc, total - pb - pc))
        })
        .collect::<BTreeMap<_, _>>();

    let ans = *(0..1 << 9)
        .rev()
        .skip(1)
        .map(|s| {
            let mut bs = BitSet::new(9);
            bs.buffer_mut()[0] = s;
            (s, bs)
        })
        .fold(
            points,
            |points: BTreeMap<(u64, u64), (usize, usize)>, (s, bs)| {
                successors(Some(s), |t| t.checked_sub(1).map(|t| t & s))
                    .filter(|t| {
                        s.count_ones() == 2 * t.count_ones()
                            || s.count_ones() == 2 * t.count_ones() + 1
                    })
                    .map(|t| {
                        let mut bs2 = BitSet::new(9);
                        bs2.buffer_mut()[0] = t;
                        (t, bs2)
                    })
                    .fold(points, |points, (t, _bs2)| {
                        let p = if s.count_ones() % 2 == 0 {
                            (0..9)
                                .filter(|&i| !bs[i])
                                .map(|i| *points.get(&(s ^ (1 << i), t)).unwrap())
                                .max_by_key(|&(p0, _p1)| p0)
                                .unwrap()
                        } else {
                            (0..9)
                                .filter(|&i| !bs[i])
                                .map(|i| *points.get(&(s ^ (1 << i), t ^ (1 << i))).unwrap())
                                .max_by_key(|&(_p0, p1)| p1)
                                .unwrap()
                        };
                        inserted!(points, (s, t), p)
                    })
            },
        )
        .get(&(0, 0))
        .unwrap();
    println!("{}", ans.0);
    println!("{}", ans.1);
}
