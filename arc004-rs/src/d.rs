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

const M: usize = 1_000_000_007;

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
impl std::ops::Neg for Mod {
    type Output = Self;
    fn neg(self) -> Self {
        Mod::new(M - self.0)
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
impl std::ops::Mul<usize> for Mod {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self {
        Mod::new(self.0 * rhs)
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

fn main() {
    let (n, m) = read_tuple!(i64, usize);

    let (n, is_negative) = if n > 0 {
        (n as usize, 0)
    } else {
        (-n as usize, 1)
    };

    let mut pf = (2..)
        .scan(n, |k, i| {
            let (d, kk) = successors(Some(*k), |k| Some(k / i))
                .take_while(|&k| k % i == 0)
                .enumerate()
                .last()
                .map(|(j, k)| (j + 1, k / i))
                .unwrap_or((0, *k));
            *k = kk;
            Some((i, d, kk))
        })
        // .inspect(|tmp| eprintln!("{:?}", tmp))
        .take_while(|&(i, d, k)| d > 0 || i * i <= k)
        .filter(|&(_, d, _)| d > 0)
        .collect_vec();
    if pf.is_empty() {
        pf.push((n, 1, 1));
    } else if pf[pf.len() - 1].2 > 1 {
        pf.push((pf[pf.len() - 1].2, 1, 1));
    }

    let fact: Vec<_> = std::iter::once(Mod::one())
        .chain((1..=m + 100).scan(Mod::one(), |f, i| {
            *f = *f * i;
            Some(*f)
        }))
        .collect();
    let inv = (2..=m + 100).fold(vec![Mod::one(), Mod::one()], |mut inv, i| {
        inv.push(-Mod::new(M / i) * inv[M % i]);
        inv
    });
    let inv_fact: Vec<_> = inv
        .iter()
        .copied()
        .scan(Mod::one(), |f, i| {
            *f = *f * i;
            Some(*f)
        })
        .collect();

    let pos_ans = pf
        .iter()
        .copied()
        .map(|(_p, d, _k)| d)
        .map(|d| {
            // C(d + m - 1, d)
            fact[d + m - 1] * inv_fact[d] * inv_fact[m - 1]
        })
        .product::<Mod>();
    // for negative solutions
    let ans = (0..=m)
        .filter(|i| i % 2 == is_negative)
        .map(|i| {
            // C(m, i)
            pos_ans * fact[m] * inv_fact[i] * inv_fact[m - i]
        })
        .sum::<Mod>();
    println!("{}", ans);
}
