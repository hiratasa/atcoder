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

#[allow(dead_code)]
fn invmod(a: usize, m: usize) -> Option<usize> {
    let (_zero, g, _u, v) =
        std::iter::successors(Some((a as i64, m as i64, 1, 0)), |&(a, b, u, v)| {
            if a == 0 {
                None
            } else {
                Some((b % a, a, -u * (b / a) + v, u))
            }
        })
        .last()
        .unwrap();

    if g == 1 {
        // |v| < m が保障される
        if v < 0 {
            Some((v + m as i64) as usize)
        } else {
            Some(v as usize)
        }
    } else {
        None
    }
}

fn crt(rm: &[(usize, usize)]) -> usize {
    rm.citer()
        .fold((0usize, 1usize), |(r0, m0), (r1, m1)| {
            let x = invmod(m0, m1).unwrap();

            let l = m0 * m1;

            ((r0 + (l + r1 - r0) % m1 * x % m1 * m0) % l, l)
        })
        .0
}

#[allow(dead_code)]
fn simulate(n: usize, a: &[usize]) -> Vec<usize> {
    let l = a.len();
    let mut nexts = vec![vec![0; l]; 30];
    nexts[0] = a.citer().map(|x| x - 1).collect::<Vec<_>>();
    for i in 1..30 {
        for j in 0..l {
            nexts[i][j] = nexts[i - 1][nexts[i - 1][j]];
        }
    }

    (0..l)
        .map(|mut i| {
            let mut nn = n;
            for j in (0..30).rev() {
                if nn >= 1 << j {
                    i = nexts[j][i];
                    nn -= 1 << j;
                }
            }
            i
        })
        .map(|i| i + 1)
        .collect()
}

fn main() {
    let p = vec![2 * 2, 3 * 3, 5, 7, 11, 13, 17, 19, 23];

    let g = izip!(
        p.citer().flat_map(|x| { (1..x).chain(once(0)) }),
        p.citer()
            .scan(0, |s, x| {
                *s += x;
                Some((*s - x, x))
            })
            .flat_map(|(s, x)| { itertools::repeat_n(s, x) })
    )
    .map(|(offset, x)| offset + x)
    .collect::<Vec<_>>();

    let a = g.citer().map(|x| x + 1).collect::<Vec<_>>();

    println!("{}", a.len());
    println!("{}", a.citer().join(" "));

    let b = read_row::<usize>();
    // let b = simulate(1000000000, &a);

    let rm = izip!(
        once(0)
            .chain(p.citer())
            .cumsum::<usize>()
            .take(p.len())
            .map(|i| { b[i] - 1 - i }),
        p.citer()
    )
    .collect::<Vec<_>>();

    let ans = crt(&rm);

    println!("{}", ans);
}
