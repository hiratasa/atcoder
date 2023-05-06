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

fn calc(i: i64, j: i64) -> usize {
    let dist = |x: i64| {
        if x % 3 == 0 {
            usize::MAX
        } else if x >= 0 {
            (x / 3 * 2 + x % 3 - 1) as usize
        } else {
            let xx = x.abs();
            (xx / 3 * 2 + xx % 3) as usize
        }
    };

    if i == 1 && j == 1 {
        0
    } else if i == 2 && j == 2 {
        1
    } else if i == j {
        max(dist(i), dist(j)).saturating_add(1)
    } else {
        max(dist(i), dist(j))
    }
}

// 検証用
#[allow(dead_code)]
fn check0() {
    const N: usize = 20;

    let mut costs = vec![vec![usize::MAX; 2 * N + 1]; 2 * N + 1];
    let mut q = VecDeque::new();

    costs[N + 1][N + 1] = 0;
    q.push_back((N + 1, N + 1));

    while let Some((i, j)) = q.pop_front() {
        iproduct!(-2i32..=2, -2i32..=2)
            .filter(|&(di, dj)| di.abs() + dj.abs() <= 3 && ((di + 3) % 3, (dj + 3) % 3) != (0, 0))
            .map(|(di, dj)| ((i as i32 + di) as usize, (j as i32 + dj) as usize))
            .filter(|&(ni, nj)| ni < 2 * N + 1 && nj < 2 * N + 1)
            .filter(|&(ni, nj)| ni % 3 != N % 3 && nj % 3 != N % 3)
            .for_each(|(ni, nj)| {
                if costs[i][j] + 1 < costs[ni][nj] {
                    costs[ni][nj] = costs[i][j] + 1;
                    q.push_back((ni, nj));
                }
            });
    }

    for i in (0..2 * N + 1).rev() {
        for j in 0..2 * N + 1 {
            let expected = calc(i as i64 - N as i64, j as i64 - N as i64);

            assert_eq!(
                costs[i][j],
                expected,
                "{} {}",
                i as i32 - N as i32,
                j as i32 - N as i32
            );

            if costs[i][j] == usize::MAX {
                print!("__ ");
            } else {
                print!("{:>2} ", costs[i][j]);
            }
        }
        println!();
    }
}

fn main() {
    let t = read::<usize>();
    let query = read_vec(t, || read_tuple!(i64, i64, i64, i64, i64, i64));

    query
        .citer()
        .map(|(ax, ay, bx, by, cx, cy)| calc(ax + bx + cx, ay + by + cy))
        .for_each(|ans| {
            println!("{}", ans);
        });
}
