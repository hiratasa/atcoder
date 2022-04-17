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

const DEG: usize = 6;

#[derive(Debug, Clone, Copy)]
struct Polynomial {
    coeff: [Mod1000000007; DEG + 1],
}

impl Polynomial {
    fn new(coeff: &[Mod1000000007]) -> Polynomial {
        assert!(coeff.len() <= DEG + 1);

        let mut c = [Mod::zero(); DEG + 1];
        for i in 0..coeff.len() {
            c[i] = coeff[i];
        }

        Polynomial { coeff: c }
    }

    fn zero() -> Polynomial {
        Polynomial::new(&[Mod::zero()])
    }

    fn one() -> Polynomial {
        Polynomial::new(&[Mod::one()])
    }
}

impl std::ops::Mul<Mod1000000007> for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Mod1000000007) -> Self::Output {
        let mut coeff = [Mod::zero(); DEG + 1];
        for i in 0..=DEG {
            coeff[i] = self.coeff[i] * rhs;
        }

        Polynomial::new(&coeff)
    }
}

impl std::ops::Mul<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut coeff = [Mod::zero(); DEG + 1];
        for i in 0..=DEG {
            for j in 0..=DEG {
                if i + j > DEG {
                    assert!(self.coeff[i].is_zero() || rhs.coeff[j].is_zero());
                    continue;
                }
                coeff[i + j] = coeff[i + j] + self.coeff[i] * rhs.coeff[j];
            }
        }

        Polynomial::new(&coeff)
    }
}

impl std::ops::Add<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Self) -> Self::Output {
        let mut coeff = [Mod::zero(); DEG + 1];
        for i in 0..=DEG {
            coeff[i] = self.coeff[i] + rhs.coeff[i];
        }

        Polynomial::new(&coeff)
    }
}

impl std::ops::Div<usize> for Polynomial {
    type Output = Polynomial;

    fn div(self, rhs: usize) -> Self::Output {
        let mut coeff = self.coeff.clone();

        for i in 0..=DEG {
            coeff[i] /= rhs;
        }

        Polynomial::new(&coeff)
    }
}

// sum[t=x to upper] t^deg
fn sum0_impl(deg: usize, upper: usize) -> Polynomial {
    let u = Mod::new(upper);
    match deg {
        0 => {
            // u + 1 - x
            Polynomial::new(&[u + 1, -Mod::one()])
        }
        1 => {
            // (u+x)*(u+1-x)/2
            Polynomial::new(&[u, Mod::one()]) * Polynomial::new(&[u + 1, -Mod::one()]) / 2
        }
        2 => {
            // (2u^2+u+2ux-x+2x^2)*(u+1-x)/6
            Polynomial::new(&[u.pow(2) * 2 + u, u * 2 - 1, Mod::new(2)])
                * Polynomial::new(&[u + 1, -Mod::one()])
                / 6
        }
        3 => {
            // (u^2+u-x+x^2)*(u+x)*(u+1-x)/4
            Polynomial::new(&[u.pow(2) + u, -Mod::one(), Mod::one()])
                * Polynomial::new(&[u, Mod::one()])
                * Polynomial::new(&[u + 1, -Mod::one()])
                / 4
        }
        4 => {
            // (u^5/5+u^4/2+u^3/3-u/30 + x/30 - x^3/3 + x^4/2 - x^5/5)
            Polynomial::new(&[
                u.pow(5) / 5 + u.pow(4) / 2 + u.pow(3) / 3 - u / 30,
                Mod::one() / 30,
                Mod::zero(),
                -Mod::one() / 3,
                Mod::one() / 2,
                -Mod::one() / 5,
            ])
        }
        5 => {
            // (2u^6+6u^5+5u^4-u^2 + x^2 - 5x^4 + 6x^5 - 2x^6) / 12
            Polynomial::new(&[
                u.pow(6) * 2 + u.pow(5) * 6 + u.pow(4) * 5 - u.pow(2),
                Mod::zero(),
                Mod::one(),
                Mod::zero(),
                -Mod::new(5),
                Mod::new(6),
                -Mod::new(2),
            ]) / 12
        }
        _ => unreachable!(),
    }
}

// sum[t=x+1 to upper] t^deg
fn sum1_impl(deg: usize, upper: usize) -> Polynomial {
    let u = Mod::new(upper);
    match deg {
        0 => {
            // u - x
            Polynomial::new(&[u, -Mod::one()])
        }
        1 => {
            // (u+1+x)*(u-x)/2
            Polynomial::new(&[u + 1, Mod::one()]) * Polynomial::new(&[u, -Mod::one()]) / 2
        }
        2 => {
            // (2u^2+3u+1+2ux+3x+2x^2)*(u-x)/6
            Polynomial::new(&[u.pow(2) * 2 + u * 3 + 1, u * 2 + 3, Mod::new(2)])
                * Polynomial::new(&[u, -Mod::one()])
                / 6
        }
        3 => {
            // (u^2+u+x+x^2)*(u+1+x)*(u-x)/4
            Polynomial::new(&[u.pow(2) + u, Mod::one(), Mod::one()])
                * Polynomial::new(&[u + 1, Mod::one()])
                * Polynomial::new(&[u, -Mod::one()])
                / 4
        }
        4 => {
            // (u^5/5+u^4/2+u^3/3-u/30 + x/30 - x^3/3 - x^4/2 - x^5/5)
            Polynomial::new(&[
                u.pow(5) / 5 + u.pow(4) / 2 + u.pow(3) / 3 - u / 30,
                Mod::one() / 30,
                Mod::zero(),
                -Mod::one() / 3,
                -Mod::one() / 2,
                -Mod::one() / 5,
            ])
        }
        5 => {
            // (2u^6+6u^5+5u^4-u^2 + x^2 - 5x^4 - 6x^5 - 2x^6) / 12
            Polynomial::new(&[
                u.pow(6) * 2 + u.pow(5) * 6 + u.pow(4) * 5 - u.pow(2),
                Mod::zero(),
                Mod::one(),
                Mod::zero(),
                -Mod::new(5),
                -Mod::new(6),
                -Mod::new(2),
            ]) / 12
        }
        _ => unreachable!(),
    }
}

// sum[t=x to upper] poly
fn sum0(poly: &Polynomial, upper: usize) -> Polynomial {
    assert!(poly.coeff[DEG].is_zero());
    (0..DEG)
        .map(|i| sum0_impl(i, upper) * poly.coeff[i])
        .fold(Polynomial::new(&[]), |ret, q| ret + q)
}

// sum[t=x+1 to upper] poly
fn sum1(poly: &Polynomial, upper: usize) -> Polynomial {
    assert!(poly.coeff[DEG].is_zero());
    (0..DEG)
        .map(|i| sum1_impl(i, upper) * poly.coeff[i])
        .fold(Polynomial::new(&[]), |ret, q| ret + q)
}

fn len_lis(t: &[(usize, bool)]) -> usize {
    let n = t.len();

    let mut u = vec![1; n];
    for i in 1..n {
        if t[i].1 {
            u[t[i].0] = u[t[i - 1].0];
        } else {
            u[t[i].0] = u[t[i - 1].0] + 1;
        }
    }

    let mut dp = vvec![0; usize::MAX; n + 1];
    for i in 0..n {
        let j = (0..n).find(|&j| dp[j] < u[i] && u[i] <= dp[j + 1]).unwrap();
        dp[j + 1] = u[i];
    }

    dp.citer().take_while(|&x| x != usize::MAX).count() - 1
}

fn solve(a: &[usize], t: &mut Vec<(usize, bool)>, used: &mut [bool]) -> Mod1000000007 {
    let n = a.len();

    if t.len() == n {
        let l = len_lis(&t);

        let (poly, upper) =
            t.citer()
                .rev()
                .fold((Polynomial::one(), usize::MAX), |(poly, upper), (i, eq)| {
                    if eq {
                        (poly, min(upper, a[i] + 1))
                    } else {
                        (
                            sum1(&poly, min(upper.saturating_sub(1), a[i])),
                            min(upper.saturating_sub(1), a[i]),
                        )
                    }
                });

        return if upper == 0 {
            Mod::zero()
        } else {
            poly.coeff[0] * l
        };
    }

    // eq
    let z0 = if !t.is_empty() {
        let last = t[t.len() - 1].0;
        (0..n)
            .map(|i| {
                if (a[last], last) < (a[i], i) && !used[i] {
                    used[i] = true;
                    t.push((i, true));
                    let x = solve(&a, t, used);
                    t.pop();
                    used[i] = false;

                    x
                } else {
                    Mod::zero()
                }
            })
            .sum::<Mod<_>>()
    } else {
        Mod::zero()
    };

    // greater
    let z1 = (0..n)
        .map(|i| {
            if !used[i] {
                used[i] = true;
                t.push((i, false));
                let x = solve(&a, t, used);
                t.pop();
                used[i] = false;

                x
            } else {
                Mod::zero()
            }
        })
        .sum::<Mod<_>>();

    z0 + z1
}

fn main() {
    let n = read::<usize>();

    let a = read_row::<usize>();

    let t = a.citer().map(|aa| Mod::new(aa)).product::<Mod1000000007>();

    let ans = solve(&a, &mut vec![], &mut vec![false; n]) / t;

    println!("{}", ans);
}
