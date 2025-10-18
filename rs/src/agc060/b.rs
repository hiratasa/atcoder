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

// fn count(grid: &[Vec<usize>], i: usize, j: usize, x: usize) -> usize {
//     let n = grid.len();
//     let m = grid[0].len();

//     let x = x ^ grid[i][j];

//     if i == n - 1 && j == m - 1 {
//         return (x == 0) as usize;
//     }

//     let r0 = if i < n - 1 {
//         count(grid, i + 1, j, x)
//     } else {
//         0
//     };

//     let r1 = if j < m - 1 {
//         count(grid, i, j + 1, x)
//     } else {
//         0
//     };

//     r0 + r1
// }

// fn solve0(n: usize, m: usize, k: usize, s: &[char]) -> bool {
//     (0..n * m)
//         .map(|_| (0..1 << k))
//         .multi_cartesian_product()
//         .any(|v| {
//             let mut grid = vec![vec![0; m]; n];
//             for i in 0..n {
//                 for j in 0..m {
//                     grid[i][j] = v[i * m + j];
//                 }
//             }

//             let x = once((0, 0))
//                 .chain(s.citer().scan((0, 0), |(i, j), c| {
//                     if c == 'D' {
//                         *i += 1;
//                     } else {
//                         *j += 1;
//                     }
//                     Some((*i, *j))
//                 }))
//                 .map(|(i, j)| grid[i][j])
//                 .fold(0, |x, y| x ^ y);

//             if x != 0 {
//                 return false;
//             }

//             if count(&grid, 0, 0, 0) != 1 {
//                 return false;
//             }

//             true
//         })
// }

fn main() {
    let t = read::<usize>();
    for _ in 0..t {
        let (n, m, k) = read_tuple!(usize, usize, usize);
        let s = read_str();

        let b = s
            .citer()
            .group_by(|&c| c)
            .into_iter()
            .map(|(_, it)| it.count())
            .collect::<Vec<_>>();
        let x = b[0..b.len() - 1]
            .citer()
            .scan(true, |t, l| {
                if *t {
                    *t = false;
                    Some(1)
                } else if l == 1 {
                    *t = true;
                    Some(0)
                } else {
                    Some(1)
                }
            })
            .sum::<usize>();

        if x <= k {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
