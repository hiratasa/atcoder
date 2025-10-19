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

#[allow(dead_code)]
pub fn pow_mod(mut x: usize, mut p: usize, m: usize) -> usize {
    let mut y = 1;
    while p > 0 {
        if p & 1 > 0 {
            y = y * x % m;
        }
        x = x * x % m;
        p >>= 1;
    }
    y
}

fn main() {
    let n: usize = read();
    let a = read_row::<usize>();

    let x = a
        .citer()
        .scan(0usize, |x, aa| {
            *x ^= aa;
            Some(*x)
        })
        .collect_vec();

    let z = once(0)
        .chain(x.citer().map(|xx| (xx == 0) as usize))
        .cumsum::<usize>()
        .collect_vec();

    const M: usize = 1_000_000_007;

    eprintln!("{:?}", x);

    let ans = if x[n - 1] == 0 {
        pow_mod(2, z[n] - 1, M)
            + x.citer()
                .enumerate()
                .filter(|t| t.1 > 0)
                .fold(FxHashMap::default(), |mut map, (i, xx)| {
                    if let Some((j, k0, kx)) = map.get_mut(&xx) {
                        *k0 += (z[i] - z[*j]) * *kx % M;
                        *k0 %= M;
                        *kx += *k0;
                        *kx %= M;
                        *j = i;
                    } else {
                        map.insert(xx, (i, 1, 1));
                    }

                    map
                })
                .into_iter()
                .map(|t| (t.1).2)
                .fold(0usize, |acc, x| (acc + x) % M)
    } else {
        x.citer()
            .fold((1usize, 0usize), |(k0, kx), xx| {
                // eprintln!("{} {} {}", xx, k0, kx);
                if xx == 0 {
                    ((k0 + kx) % M, kx)
                } else if xx == x[n - 1] {
                    (k0, (k0 + kx) % M)
                } else {
                    (k0, kx)
                }
            })
            .0
    };
    println!("{}", ans % M);
    // eprintln!("{}", calc(&a));
}

fn calc(a: &[usize]) -> usize {
    (0..a.len() - 1)
        .map(|_| 0..2)
        .multi_cartesian_product()
        .filter(|s| {
            once(0)
                .chain(s.citer().positions(|b| b > 0).map(|idx| idx + 1))
                .chain(once(a.len()))
                .tuple_windows()
                .map(|(b, e)| a[b..e].citer().fold(0usize, |x, aa| x ^ aa))
                .all_equal()
        })
        .count()
}
