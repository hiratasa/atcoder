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

use num::{One, Zero};
#[allow(dead_code)]
pub fn pow_mod(mut x: usize, mut p: usize, m: usize) -> usize {
    let mut y = 1;
    x = x % m;
    while p > 0 {
        if p & 1 > 0 {
            y = y * x % m;
        }
        x = x * x % m;
        p >>= 1;
    }
    y
}
pub trait Modulus: Copy + Eq {
    fn modulus() -> usize;
}
macro_rules! define_static_mod {
    ($ m : expr , $ modulus : ident , $ mod : ident ) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub struct $modulus();
        impl Modulus for $modulus {
            fn modulus() -> usize {
                $m
            }
        }
        #[allow(dead_code)]
        pub type $mod = Mod<$modulus>;
    };
}
define_static_mod!(2013265921, Modulus2013265921, Mod2013265921);
define_static_mod!(1811939329, Modulus1811939329, Mod1811939329);
define_static_mod!(469762049, Modulus469762049, Mod469762049);
define_static_mod!(998244353, Modulus998244353, Mod998244353);
define_static_mod!(1224736769, Modulus1224736769, Mod1224736769);
define_static_mod!(1000000007, Modulus1000000007, Mod1000000007);
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mod<M>(pub usize, std::marker::PhantomData<fn() -> M>);
#[allow(dead_code)]
impl<M: Modulus> Mod<M> {
    pub fn modulus() -> usize {
        M::modulus()
    }
    pub fn new(n: usize) -> Self {
        Mod(n % M::modulus(), std::marker::PhantomData)
    }
    pub fn pow(self, p: usize) -> Self {
        Mod::new(pow_mod(self.0, p, M::modulus()))
    }
    pub fn inv(self) -> Self {
        let (_zero, g, _u, v) = std::iter::successors(
            Some((self.0 as i64, M::modulus() as i64, 1, 0)),
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
        assert_eq!(
            g,
            1,
            "gcd({}, {}) must be 1 but {}.",
            self.0,
            M::modulus(),
            g
        );
        Mod::new((v + M::modulus() as i64) as usize)
    }
}
impl<M> std::fmt::Display for Mod<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<M> std::fmt::Debug for Mod<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<T, M: Modulus> std::convert::From<T> for Mod<M>
where
    usize: std::convert::TryFrom<T>,
{
    fn from(v: T) -> Self {
        use std::convert::TryFrom;
        Mod::new(usize::try_from(v).ok().unwrap())
    }
}
impl<M: Modulus> std::str::FromStr for Mod<M> {
    type Err = <usize as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        usize::from_str(s).map(|n| Mod::new(n))
    }
}
impl<M: Modulus> std::ops::Neg for Mod<M> {
    type Output = Self;
    fn neg(self) -> Self {
        Mod::new(M::modulus() - self.0)
    }
}
impl<T, M: Modulus> std::ops::Add<T> for Mod<M>
where
    T: std::convert::Into<Mod<M>>,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self {
        Mod::new(self.0 + rhs.into().0)
    }
}
impl<T, M: Modulus> std::ops::AddAssign<T> for Mod<M>
where
    T: std::convert::Into<Mod<M>>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}
impl<T, M: Modulus> std::ops::Sub<T> for Mod<M>
where
    T: std::convert::Into<Mod<M>>,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self {
        Mod::new(self.0 + M::modulus() - rhs.into().0)
    }
}
impl<T, M: Modulus> std::ops::SubAssign<T> for Mod<M>
where
    T: std::convert::Into<Mod<M>>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}
impl<T, M: Modulus> std::ops::Mul<T> for Mod<M>
where
    T: std::convert::Into<Mod<M>>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Mod::new(self.0 * rhs.into().0)
    }
}
impl<T, M: Modulus> std::ops::MulAssign<T> for Mod<M>
where
    T: std::convert::Into<Mod<M>>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}
impl<T, M: Modulus> std::ops::Div<T> for Mod<M>
where
    T: std::convert::Into<Mod<M>>,
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
impl<T, M: Modulus> std::ops::DivAssign<T> for Mod<M>
where
    T: std::convert::Into<Mod<M>>,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}
impl<M: Modulus> std::iter::Product for Mod<M> {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Mod::one(), |p, a| p * a)
    }
}
impl<M: Modulus> std::iter::Sum for Mod<M> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Mod::zero(), |p, a| p + a)
    }
}
impl<M: Modulus> num::Zero for Mod<M> {
    fn zero() -> Self {
        Mod::new(0)
    }
    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
impl<M: Modulus> num::One for Mod<M> {
    fn one() -> Self {
        Mod::new(1)
    }
    fn is_one(&self) -> bool {
        self.0 == 1
    }
}

fn main() {
    type Mod = Mod998244353;;

    let n = read::<usize>();
    let a = read_row::<usize>();

    const M: usize = 1000000;
    let freq = a.citer().fold(vec![Mod::zero(); M + 1], |mut freq, x| {
        freq[x] += 1;
        freq
    });

    let s = a.citer().sum::<usize>();

    let primes = (2..=M)
        .scan(vec![true; M + 1], |is_prime, p| {
            if is_prime[p] {
                (2..)
                    .map(|i| i * p)
                    .take_while(|&i| i <= M)
                    .for_each(|i| is_prime[i] = false);
                Some(Some(p))
            } else {
                Some(None)
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut t = (0..=M).map(|i| freq[i] * i).collect::<Vec<_>>();
    // zeta
    for &p in &primes {
        (1..=M / p).rev().for_each(|i| {
            t[i] = t[i] + t[i * p];
        });
    }

    t.iter_mut().for_each(|x| *x = *x * *x);

    // mebius
    for &p in &primes {
        (1..=M).take_while(|&i| i * p <= M).for_each(|i| {
            t[i] = t[i] - t[i * p];
        });
    }

    let ans = ((1..=M).map(|i| t[i] / i).sum::<Mod>() - s) / 2;

    println!("{}", ans);
}