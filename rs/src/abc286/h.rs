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

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct Vector(f64, f64);

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector(self * rhs.0, self * rhs.1)
    }
}

impl Vector {
    pub const EPS: f64 = 1e-10;

    pub fn dot(self, rhs: Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }

    pub fn cross(self, rhs: Self) -> f64 {
        self.0 * rhs.1 - rhs.0 * self.1
    }

    pub fn norm(self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }

    pub fn distance(self, rhs: Self) -> f64 {
        (self - rhs).norm()
    }

    pub fn is_zero(self) -> bool {
        self.norm() < Self::EPS
    }

    pub fn is_parallel(self, rhs: Self) -> bool {
        self.is_zero()
            || rhs.is_zero()
            || self.cross(rhs).abs() < Self::EPS * f64::max(self.norm(), rhs.norm())
    }

    pub fn normalize(self) -> Option<Self> {
        let n = self.norm();
        if n == 0.0 {
            None
        } else {
            Some(Vector(self.0 / n, self.1 / n))
        }
    }
}

#[allow(dead_code)]
fn convex_hull(points: &Vec<Vector>) -> Vec<usize> {
    assert!(points.len() > 2);

    let idxs = {
        let mut idxs = (0..points.len()).collect::<Vec<_>>();
        idxs.sort_by(|&idx1, &idx2| points[idx1].partial_cmp(&points[idx2]).unwrap());
        idxs
    };

    let lower_ch =
        idxs.iter()
            .map(|&i| (i, &points[i]))
            .fold(vec![], |mut ch: Vec<usize>, (i, p)| {
                // 凸包の辺上のものも選んでいる
                // (辺上のものを選ばない場合はcross積が0のときにもpopする)
                while ch.len() >= 2
                    && (points[ch[ch.len() - 1]] - points[ch[ch.len() - 2]])
                        .cross(*p - points[ch[ch.len() - 2]])
                        < 0.0
                {
                    ch.pop();
                }

                ch.push(i);

                ch
            });
    let t = lower_ch.len();
    let mut ch = idxs.iter().rev().map(|&i| (i, &points[i])).skip(1).fold(
        lower_ch,
        |mut ch: Vec<usize>, (i, p)| {
            while ch.len() >= t + 1
                && (points[ch[ch.len() - 1]] - points[ch[ch.len() - 2]])
                    .cross(*p - points[ch[ch.len() - 2]])
                    < 0.0
            {
                ch.pop();
            }

            ch.push(i);

            ch
        },
    );
    ch.pop();
    ch
}

fn main() {
    let n = read::<usize>();
    let xy = read_vec(n, || read_tuple!(f64, f64));
    let s = read_tuple!(f64, f64);
    let t = read_tuple!(f64, f64);

    let v = xy
        .citer()
        .chain(once(s))
        .chain(once(t))
        .map(|(x, y)| Vector(x, y))
        .collect::<Vec<_>>();

    let ch = convex_hull(&v);

    let l = ((s.0 - t.0).powi(2) + (s.1 - t.1).powi(2)).sqrt();
    let i0 = if let Some(i0) = ch.citer().position(|idx| idx == n) {
        i0
    } else {
        println!("{}", l);
        return;
    };
    let i1 = if let Some(i1) = ch.citer().position(|idx| idx == n + 1) {
        i1
    } else {
        println!("{}", l);
        return;
    };

    let (i0, i1) = (min(i0, i1), max(i0, i1));

    let ans0 = ch[i0..=i1]
        .citer()
        .map(|idx| v[idx])
        .tuple_windows()
        .map(|(v0, v1)| v0.distance(v1))
        .sum::<f64>();
    let ans1 = chain(ch[i1..].citer(), ch[..=i0].citer())
        .map(|idx| v[idx])
        .tuple_windows()
        .map(|(v0, v1)| v0.distance(v1))
        .sum::<f64>();

    let ans = f64::min(ans0, ans1);

    println!("{}", ans);
}
