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

use num::{One, Zero};
fn modulus() -> usize {
    1_000_000_007
}
#[derive(Clone, Copy, Debug)]
struct Mod(usize);
#[allow(dead_code)]
impl Mod {
    fn new(n: usize) -> Self {
        Mod(n % modulus())
    }
    fn pow(self, p: usize) -> Self {
        if p == 0 {
            Mod::new(1)
        } else if p == 1 {
            self
        } else {
            let r = self.pow(p / 2);
            if p % 2 == 0 { r * r } else { r * r * self }
        }
    }
    fn inv(self) -> Self {
        let (_zero, g, _u, v) = std::iter::successors(
            Some((self.0 as i64, modulus() as i64, 1, 0)),
            |&(a, b, u, v)| {
                if a == 0 {
                    None
                } else {
                    Some((b % a, a, -u * (b / a) + v, u))
                }
            },
        )
        .last()
        .unwrap();
        assert_eq!(g, 1, "gcd({}, {}) must be 1 but {}.", self.0, modulus(), g);
        Mod::new((v + modulus() as i64) as usize)
    }
}
impl std::fmt::Display for Mod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<T> std::convert::From<T> for Mod
where
    usize: std::convert::From<T>,
{
    fn from(v: T) -> Mod {
        Mod::new(usize::from(v))
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
        Mod::new(modulus() - self.0)
    }
}
impl<T> std::ops::Add<T> for Mod
where
    T: std::convert::Into<Mod>,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self {
        Mod::new(self.0 + rhs.into().0)
    }
}
impl<T> std::ops::AddAssign<T> for Mod
where
    T: std::convert::Into<Mod>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}
impl<T> std::ops::Sub<T> for Mod
where
    T: std::convert::Into<Mod>,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self {
        Mod::new(self.0 + modulus() - rhs.into().0)
    }
}
impl<T> std::ops::SubAssign<T> for Mod
where
    T: std::convert::Into<Mod>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}
impl<T> std::ops::Mul<T> for Mod
where
    T: std::convert::Into<Mod>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Mod::new(self.0 * rhs.into().0)
    }
}
impl<T> std::ops::MulAssign<T> for Mod
where
    T: std::convert::Into<Mod>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}
impl<T> std::ops::Div<T> for Mod
where
    T: std::convert::Into<Mod>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        if self.0 == 0 {
            self
        } else {
            self * rhs.into().inv()
        }
    }
}
impl<T> std::ops::DivAssign<T> for Mod
where
    T: std::convert::Into<Mod>,
{
    fn div_assign(&mut self, rhs: T) {
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
impl num::Zero for Mod {
    fn zero() -> Self {
        Mod::new(0)
    }
    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
impl num::One for Mod {
    fn one() -> Self {
        Mod::new(1)
    }
    fn is_one(&self) -> bool {
        self.0 == 1
    }
}

fn main() {
    let n: usize = read();
    let a = read_row::<i64>()
        .into_iter()
        .map(|aa| if aa < 0 { None } else { Some(aa as usize) })
        .collect_vec();

    let ans = a
        .citer()
        .group_by(|&aa| aa)
        .into_iter()
        .map(|(aa, it)| (aa, it.count()))
        .tuple_windows()
        .fold(Mod::one(), |ans, (prev, current, next)| {
            if current.0.is_some() {
                ans
            } else {
                let prev = prev.0.unwrap();
                let next = next.0.unwrap();
                let c = current.1;

                // C(next - prev + c, c)
                (1..=c).fold(ans, |ans, i| ans * (next - prev + c + 1 - i) / i)
            }
        });

    println!("{}", ans);
}
