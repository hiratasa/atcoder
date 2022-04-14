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

#[allow(dead_code)]
fn lower_bound<T, F>(mut begin: T, mut end: T, epsilon: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    let two = T::try_from(2).ok().unwrap();
    while end - begin >= epsilon {
        let mid = begin + (end - begin) / two;
        match f(mid) {
            std::cmp::Ordering::Less => {
                begin = mid + epsilon;
            }
            _ => {
                end = mid;
            }
        }
    }
    begin
}
#[allow(dead_code)]
fn lower_bound_int<T, F>(begin: T, end: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    lower_bound(begin, end, T::try_from(1).ok().unwrap(), f)
}

#[allow(dead_code)]
fn solve0(t: &[Vec<Option<(usize, usize)>>], d: usize) -> usize {
    (1..)
        .find(|&k| {
            iproduct!(0..d, 0..d).any(|(i, j)| {
                iproduct!(0..d, 0..d).all(|(i1, j1)| {
                    if let Some((a, b)) = t[i1][j1] {
                        let di = (i1 + d - i) % d;
                        let dj = (j1 + d - j) % d;

                        (di + a <= k && dj + b <= k) || (di + b <= k && dj + a <= k)
                    } else {
                        true
                    }
                })
            })
        })
        .unwrap()
}

fn main() {
    let (n, d) = read_tuple!(usize, usize);
    let xy = read_vec(n, || read_tuple!(usize, usize));

    let c = xy.citer().fold(vec![vec![0; d]; d], |mut c, (x, y)| {
        c[x % d][y % d] += 1;
        c
    });

    let t = c
        .iter()
        .map(|row| {
            row.citer()
                .map(|cc| {
                    let a = lower_bound_int(0, 1000, |a| (a * a).cmp(&cc));

                    if a > 0 && (a - 1) * a >= cc {
                        Some(((a - 1) * d, (a - 2) * d))
                    } else if a > 0 {
                        Some(((a - 1) * d, (a - 1) * d))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (a, _b) = t
        .iter()
        .map(|row| row.citer().max().unwrap())
        .max()
        .unwrap()
        .unwrap();

    let poss0 = iproduct!(0..d, 0..d)
        .filter(|&(x, y)| t[x][y] == Some((a, a)))
        .collect::<Vec<_>>();
    let m0 = poss0.len();
    let poss1 = iproduct!(0..d, 0..d)
        .filter(|&(x, y)| matches!(t[x][y], Some((aa, _)) if aa == a))
        .collect::<Vec<_>>();
    let m1 = poss1.len();

    let mut mat0 = poss0
        .citer()
        .fold(vec![vec![0; 2 * d + 1]; 2 * d + 1], |mut mat, (i, j)| {
            mat[i + 1][j + 1] += 1;
            mat[i + 1][j + d + 1] += 1;
            mat[i + d + 1][j + 1] += 1;
            mat[i + d + 1][j + d + 1] += 1;

            mat
        });
    let mut mat1 = poss1
        .citer()
        .fold(vec![vec![0; 2 * d + 1]; 2 * d + 1], |mut mat, (i, j)| {
            mat[i + 1][j + 1] += 1;
            mat[i + 1][j + d + 1] += 1;
            mat[i + d + 1][j + 1] += 1;
            mat[i + d + 1][j + d + 1] += 1;

            mat
        });
    for i in 0..=2 * d {
        for j in 1..=2 * d {
            mat0[i][j] += mat0[i][j - 1];
            mat1[i][j] += mat1[i][j - 1];
        }
    }
    for i in 1..=2 * d {
        for j in 0..=2 * d {
            mat0[i][j] += mat0[i - 1][j];
            mat1[i][j] += mat1[i - 1][j];
        }
    }

    let z = lower_bound_int(0, d, |z| {
        let ok = iproduct!(0..d, 0..d).any(|(i, j)| {
            mat0[i + z + 1][j + z + 1] + mat0[i][j] - mat0[i + z + 1][j] - mat0[i][j + z + 1] == m0
                && mat1[i + z + 1][j + d] + mat1[i][j] - mat1[i][j + d] + mat1[i + d][j + z + 1]
                    - mat1[i + d][j]
                    - mat1[i + z + 1][j + z + 1]
                    == m1
        });

        if ok {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    let ans = a + z;

    println!("{}", ans);
}
