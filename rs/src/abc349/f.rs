use std::{iter::successors, mem::replace};

use proconio::input;

fn main() {
    type Mod = Mod998244353;

    input! {
        n: usize, m: usize,
        a: [usize; n],
    };

    let factors = (2..)
        .scan(m, |m, k| {
            if *m == 1 {
                None
            } else if k * k > *m {
                Some((replace(m, 1), 1))
            } else {
                let d;
                (d, *m) = successors(Some(*m), |&l| Some(l / k))
                    .enumerate()
                    .find(|&(_, l)| l % k > 0)
                    .unwrap();

                Some((k, d))
            }
        })
        .filter(|&(_, d)| d > 0)
        .collect::<Vec<_>>();

    let a = a
        .into_iter()
        .filter_map(|mut x| {
            let mut b = 0usize;

            for (i, &(p, d)) in factors.iter().enumerate() {
                let mut j = 0;
                while x % p == 0 {
                    x /= p;
                    j += 1;
                }

                if j == d {
                    b |= 1 << i;
                } else if j > d {
                    return None;
                }
            }

            if x > 1 {
                return None;
            }

            Some(b)
        })
        .collect::<Vec<_>>();

    let l = factors.len();
    let mut t = vec![0; 1 << l];
    for &x in &a {
        t[x] += 1;
    }

    for i in 0..l {
        for s in 0..1 << l {
            if s & (1 << i) == 0 {
                t[s | (1 << i)] = t[s | (1 << i)] + t[s];
            }
        }
    }

    let mut ans = Mod::zero();
    for s in 0usize..1 << l {
        let num = t[((1 << l) - 1) ^ s];
        let y = Mod::new(2).pow(num) - 1;

        if s.count_ones() % 2 == 0 {
            ans += y;
        } else {
            ans -= y;
        }
    }

    println!("{ans}");
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
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct StaticModulus<const M: usize>();
impl<const M: usize> Modulus for StaticModulus<M> {
    fn modulus() -> usize {
        M
    }
}
macro_rules! define_static_mod {
    ($ m : expr , $ mod : ident ) => {
        #[allow(dead_code)]
        pub type $mod = Mod<StaticModulus<$m>>;
    };
}
define_static_mod!(2013265921, Mod2013265921);
define_static_mod!(1811939329, Mod1811939329);
define_static_mod!(469762049, Mod469762049);
define_static_mod!(998244353, Mod998244353);
define_static_mod!(1224736769, Mod1224736769);
define_static_mod!(1000000007, Mod1000000007);
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
    pub fn raw(n: usize) -> Self {
        Mod(n, std::marker::PhantomData)
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
    T: num::traits::Unsigned,
{
    fn from(v: T) -> Self {
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
        if self.0 == 0 {
            Mod::raw(0)
        } else {
            Mod::raw(M::modulus() - self.0)
        }
    }
}
impl<M: Modulus> std::ops::Add<Mod<M>> for Mod<M> {
    type Output = Self;
    fn add(self, rhs: Mod<M>) -> Self {
        let r = self.0 + rhs.0;
        if r < M::modulus() {
            Mod::raw(r)
        } else {
            Mod::raw(r - M::modulus())
        }
    }
}
impl<M: Modulus> std::ops::Add<usize> for Mod<M> {
    type Output = Self;
    fn add(self, rhs: usize) -> Self {
        self + Mod::new(rhs)
    }
}
impl<M: Modulus> std::ops::Add<Mod<M>> for usize {
    type Output = Mod<M>;
    fn add(self, rhs: Mod<M>) -> Mod<M> {
        Mod::new(self) + rhs.0
    }
}
impl<T, M: Modulus> std::ops::AddAssign<T> for Mod<M>
where
    Mod<M>: std::ops::Add<T, Output = Mod<M>>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}
impl<M: Modulus> std::ops::Sub<Mod<M>> for Mod<M> {
    type Output = Self;
    fn sub(self, rhs: Mod<M>) -> Self {
        let r = self.0.wrapping_sub(rhs.0);
        if r < M::modulus() {
            Mod::raw(r)
        } else {
            Mod::raw(r.wrapping_add(M::modulus()))
        }
    }
}
impl<M: Modulus> std::ops::Sub<usize> for Mod<M> {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self {
        self - Mod::new(rhs)
    }
}
impl<M: Modulus> std::ops::Sub<Mod<M>> for usize {
    type Output = Mod<M>;
    fn sub(self, rhs: Mod<M>) -> Mod<M> {
        Mod::new(self) - rhs
    }
}
impl<T, M: Modulus> std::ops::SubAssign<T> for Mod<M>
where
    Mod<M>: std::ops::Sub<T, Output = Mod<M>>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}
impl<M: Modulus> std::ops::Mul<Mod<M>> for Mod<M> {
    type Output = Self;
    fn mul(self, rhs: Mod<M>) -> Self {
        Mod::new(self.0 * rhs.0)
    }
}
impl<M: Modulus> std::ops::Mul<usize> for Mod<M> {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self {
        Mod::new(self.0 * (rhs % M::modulus()))
    }
}
impl<M: Modulus> std::ops::Mul<Mod<M>> for usize {
    type Output = Mod<M>;
    fn mul(self, rhs: Mod<M>) -> Mod<M> {
        Mod::new((self % M::modulus()) * rhs.0)
    }
}
impl<T, M: Modulus> std::ops::MulAssign<T> for Mod<M>
where
    Mod<M>: std::ops::Mul<T, Output = Mod<M>>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}
impl<M: Modulus> std::ops::Div<Mod<M>> for Mod<M> {
    type Output = Self;
    fn div(self, rhs: Mod<M>) -> Self {
        assert!(!rhs.is_zero());
        if self.0 == 0 {
            self
        } else {
            self * rhs.inv()
        }
    }
}
impl<M: Modulus> std::ops::Div<usize> for Mod<M> {
    type Output = Self;
    fn div(self, rhs: usize) -> Self {
        assert_ne!(rhs, 0);
        if self.0 == 0 {
            self
        } else {
            self * Mod::new(rhs).inv()
        }
    }
}
impl<M: Modulus> std::ops::Div<Mod<M>> for usize {
    type Output = Mod<M>;
    fn div(self, rhs: Mod<M>) -> Mod<M> {
        assert!(!rhs.is_zero());
        if self == 0 {
            Mod::new(self)
        } else {
            self * rhs.inv()
        }
    }
}
impl<T, M: Modulus> std::ops::DivAssign<T> for Mod<M>
where
    Mod<M>: std::ops::Div<T, Output = Mod<M>>,
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
impl<M: Modulus> rand::distributions::Distribution<Mod<M>> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Mod<M> {
        Mod::new(rng.gen_range(0..M::modulus()))
    }
}
