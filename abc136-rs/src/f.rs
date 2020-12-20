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
use itertools::{chain, iproduct, izip, Itertools};
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

const M: usize = 998244353;

#[derive(Clone, Copy, Debug)]
struct Mod(usize);
#[allow(dead_code)]
impl Mod {
    fn new(n: usize) -> Self {
        Mod(n % M)
    }
    fn zero() -> Self {
        Mod::new(0)
    }
    fn one() -> Self {
        Mod::new(1)
    }
    fn pow(self, p: usize) -> Self {
        if p == 0 {
            Mod::new(1)
        } else if p == 1 {
            self
        } else {
            let r = self.pow(p / 2);
            if p % 2 == 0 {
                r * r
            } else {
                r * r * self
            }
        }
    }
}
impl std::fmt::Display for Mod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for Mod {
    type Err = <usize as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        usize::from_str(s).map(|n| Mod::new(n))
    }
}
impl std::ops::Add for Mod {
    type Output = Self;
    fn add(self, rhs: Mod) -> Self {
        Mod::new(self.0 + rhs.0)
    }
}
impl std::ops::AddAssign for Mod {
    fn add_assign(&mut self, rhs: Mod) {
        *self = *self + rhs;
    }
}
impl std::ops::Sub for Mod {
    type Output = Self;
    fn sub(self, rhs: Mod) -> Self {
        Mod::new(self.0 + M - rhs.0)
    }
}
impl std::ops::SubAssign for Mod {
    fn sub_assign(&mut self, rhs: Mod) {
        *self = *self - rhs;
    }
}
impl std::ops::Mul for Mod {
    type Output = Self;
    fn mul(self, rhs: Mod) -> Self {
        Mod::new(self.0 * rhs.0)
    }
}
impl std::ops::MulAssign for Mod {
    fn mul_assign(&mut self, rhs: Mod) {
        *self = *self * rhs;
    }
}
impl std::ops::Div for Mod {
    type Output = Self;
    fn div(self, rhs: Mod) -> Self {
        if self.0 == 0 {
            self
        } else {
            assert!(rhs.0 != 0);
            self * rhs.pow(M - 2)
        }
    }
}
impl std::ops::DivAssign for Mod {
    fn div_assign(&mut self, rhs: Mod) {
        *self = *self / rhs;
    }
}
impl std::iter::Product for Mod {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Mod::one(), |p, a| p * a)
    }
}
impl std::iter::Sum for Mod {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Mod::zero(), |p, a| p + a)
    }
}

struct BIT<T> {
    len: usize,
    values: Vec<T>,
}

#[allow(dead_code)]
impl<T> BIT<T>
where
    T: std::default::Default
        + std::clone::Clone
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>,
{
    fn new(len: usize) -> BIT<T> {
        BIT {
            len,
            values: vec![T::default(); len],
        }
    }

    // [0, i)の和
    fn sum(&self, i: usize) -> T {
        let mut s = T::default();
        let mut idx = i as i64;

        // values[1] ~ values[i] の和
        // (bは1-indexedなのでこれでOK)
        while idx > 0 {
            s = s + self.values[(idx - 1) as usize].clone();
            idx -= idx & -idx;
        }

        return s;
    }

    // [i, j) の和
    fn sum_between(&self, i: usize, j: usize) -> T {
        self.sum(j) - self.sum(i)
    }

    fn add(&mut self, i: usize, a: T) {
        // 1-indexedに直す
        let mut idx = i as i64 + 1;

        while idx as usize <= self.len {
            self.values[(idx - 1) as usize] = self.values[(idx - 1) as usize].clone() + a.clone();
            idx += idx & -idx;
        }
    }

    fn len(&self) -> usize {
        self.len
    }
}

fn main() {
    let n: usize = read();

    let xy = read_vec(n, || read_tuple!(i64, i64));

    let xs = (0..n).sorted_by_key(|&i| xy[i].0).collect_vec();
    let xidxs = xs
        .iter()
        .copied()
        .enumerate()
        .fold(vec![0; n], |mut xidxs, (xidx, i)| {
            xidxs[i] = xidx;
            xidxs
        });
    let ys = (0..n).sorted_by_key(|&i| xy[i].1).collect_vec();
    let yidxs = ys
        .iter()
        .copied()
        .enumerate()
        .fold(vec![0; n], |mut yidxs, (yidx, i)| {
            yidxs[i] = yidx;
            yidxs
        });
    let two = Mod::new(2);
    let ans = ys
        .iter()
        .copied()
        .scan(BIT::new(n), |bit, i| {
            bit.add(xidxs[i], 1usize);
            let k = bit.sum(xidxs[i]);

            // subset including i
            let p0 = two.pow(n - 1);
            // subset not including i, but whose bounding box including i
            let p1 = two.pow(n - 1)
                - two.pow(xidxs[i])
                - two.pow(n - xidxs[i] - 1)
                - two.pow(yidxs[i])
                - two.pow(n - yidxs[i] - 1)
                + two.pow(k)
                + two.pow(yidxs[i] - k)
                + two.pow(xidxs[i] - k)
                + two.pow(n + k - xidxs[i] - yidxs[i] - 1)
                - Mod::one();
            Some(p0 + p1)
        })
        .sum::<Mod>();
    println!("{}", ans);
}
