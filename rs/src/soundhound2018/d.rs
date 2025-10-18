#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64;
#[allow(unused_imports)]
use std::i64;
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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom};
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
    };
    ($($x:expr),+,) => {
        it![$($x),+]
    };
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

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
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

// fn solve0(h: usize, w: usize, p: &[Vec<i64>], f: &[Vec<i64>]) -> Vec<i64> {
//     (0..h).fold(vvec![0; i64::MIN; w], |prev, i| {
//         (0..w)
//             .map(|dst| {
//                 (0..w)
//                     .map(|src| {
//                         iproduct!(0..w, 0..w)
//                             .filter(|&(j0, j1)| j0 <= j1)
//                             .map(|(j0, j1)| {
//                                 let mut visited = vec![false; w];

//                                 let x = it![src, j0, j1, dst]
//                                     .tuple_windows()
//                                     .flat_map(|(start, goal)| {
//                                         let k = max(start, goal) - min(start, goal) + 1;

//                                         (0..k).map(move |jj| {
//                                             if start <= goal {
//                                                 start + jj
//                                             } else {
//                                                 start - jj
//                                             }
//                                         })
//                                     })
//                                     .dedup()
//                                     .map(|j| {
//                                         if visited[j] {
//                                             -f[i][j]
//                                         } else {
//                                             visited[j] = true;
//                                             p[i][j] - f[i][j]
//                                         }
//                                     })
//                                     .sum::<i64>();

//                                 prev[src].saturating_add(x)
//                             })
//                             .max()
//                             .unwrap()
//                     })
//                     .max()
//                     .unwrap()
//             })
//             .collect()
//     })
// }

fn main() {
    let (h, w) = read_tuple!(usize, usize);
    let p = read_mat::<i64>(h);
    let f = read_mat::<i64>(h);

    let dp = (0..h).fold(vvec![0; i64::MIN; w], |prev, i| {
        let pfcum = once(0)
            .chain(
                izip!(p[i].citer(), f[i].citer())
                    .map(|(x, y)| x - y)
                    .cumsum::<i64>(),
            )
            .collect::<Vec<_>>();

        let leftuturn = (0..w)
            .scan((0, 0), |(cum, mi), j| {
                let r = *cum - *mi;
                if j + 1 < w {
                    *cum += p[i][j] - f[i][j] - f[i][j + 1];
                    *mi = min(*mi, *cum);
                }

                Some(r)
            })
            .collect::<Vec<_>>();
        let rightuturn = (0..w)
            .rev()
            .scan((0, 0), |(cum, mi), j| {
                let r = *cum - *mi;
                if j > 0 {
                    *cum += p[i][j] - f[i][j] - f[i][j - 1];
                    *mi = min(*mi, *cum);
                }

                Some(r)
            })
            .collect::<Vec<_>>();

        let from_left = (0..w)
            .scan(i64::MIN, |entrymax, j| {
                let entry = prev[j].saturating_add(leftuturn[j] - pfcum[j]);
                *entrymax = max(*entrymax, entry);

                Some(entrymax.saturating_add(pfcum[j + 1] + rightuturn[w - 1 - j]))
            })
            .collect::<Vec<_>>();
        let from_right = (0..w)
            .rev()
            .scan(i64::MIN, |entrymax, j| {
                let entry = prev[j].saturating_add(rightuturn[w - 1 - j] + pfcum[j + 1]);
                *entrymax = max(*entrymax, entry);

                Some(entrymax.saturating_add(-pfcum[j] + leftuturn[j]))
            })
            .collect::<Vec<_>>();

        izip!(from_left, from_right.into_iter().rev())
            .map(|(x, y)| max(x, y))
            .collect()
    });

    for ans in dp {
        println!("{}", ans);
    }
}
