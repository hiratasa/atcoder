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
use itertools::{chain, iproduct, iterate, izip, Itertools};
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

trait SliceCopiedExt<T> {
    fn citer(&self) -> std::iter::Copied<std::slice::Iter<T>>;
}

impl<V, T> SliceCopiedExt<T> for V
where
    V: std::ops::Deref<Target = [T]>,
    T: Copy,
{
    fn citer(&self) -> std::iter::Copied<std::slice::Iter<T>> {
        self.iter().copied()
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

use std::sync::atomic::{AtomicUsize, Ordering};

static M_impl: AtomicUsize = AtomicUsize::new(0usize);

fn update_M(m: usize) {
    M_impl.store(m, Ordering::Relaxed);
}

fn M() -> usize {
    M_impl.load(Ordering::Relaxed)
}

#[derive(Clone, Copy, Debug)]
struct Mod(usize);

#[allow(dead_code)]
impl Mod {
    fn new(n: usize) -> Self {
        Mod(n % M())
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

    fn inv(self) -> Self {
        let (_zero, g, _u, v) =
            std::iter::successors(Some((self.0 as i64, M() as i64, 1, 0)), |&(a, b, u, v)| {
                if a == 0 {
                    None
                } else {
                    Some((b % a, a, -u * (b / a) + v, u))
                }
            })
            .last()
            .unwrap();

        assert_eq!(g, 1, "gcd({}, {}) must be 1 but {}.", self.0, M(), g);
        // |v| < m が保障されている
        Mod::new((v + M() as i64) as usize)
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
        Mod::new(M() - self.0)
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
        Mod::new(self.0 + M() - rhs.0)
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

impl std::ops::Mul<usize> for Mod {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self {
        Mod::new(self.0 * rhs)
    }
}

impl std::ops::MulAssign<usize> for Mod {
    fn mul_assign(&mut self, rhs: usize) {
        *self = *self * rhs;
    }
}

impl std::ops::Div for Mod {
    type Output = Self;
    fn div(self, rhs: Mod) -> Self {
        if self.0 == 0 {
            self
        } else {
            self * rhs.inv()
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

#[allow(dead_code)]
fn generate_fact(n: usize) {
    let fact: Vec<_> = std::iter::once(Mod::one())
        .chain((1..=n).scan(Mod::one(), |f, i| {
            *f = *f * i;
            Some(*f)
        }))
        .collect();
    let inv = (2..=n).fold(vec![Mod::one(), Mod::one()], |mut inv, i| {
        inv.push(-Mod::new(M() / i) * inv[M() % i]);
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
}

fn main() {
    let t: usize = read();

    let nsk = read_vec(t, || read_tuple!(usize, usize, usize));

    for &(n, s, k) in &nsk {
        // s + xk = yn
        let g = gcd(k, n);

        if s % g > 0 {
            println!("{}", -1);
        } else {
            let s = s / g;
            let k = k / g;
            let n = n / g;

            update_M(n);

            // s + xk = 0 mod n
            // x = -k^-1 * s mod n
            let x = -Mod::new(s) / Mod::new(k);

            println!("{}", x);
        }
    }
}
