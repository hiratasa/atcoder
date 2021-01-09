use cargo_snippet::snippet;

#[snippet("modulo")]
use num::{One, Zero};

#[snippet("modulo")]
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

#[snippet("modulo")]
pub trait Modulus: Copy + Eq {
    fn modulus() -> usize;
}

#[snippet("modulo")]
macro_rules! define_static_mod {
    ($m:expr, $modulus:ident, $mod:ident) => {
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

#[snippet("modulo")]
define_static_mod!(998244353, Modulus998244353, Mod998244353);

#[snippet("modulo")]
define_static_mod!(1000000007, Modulus1000000007, Mod1000000007);

// for dynamic modulus
use std::sync::atomic::{AtomicUsize, Ordering};

static DYNAMIC_MODULUS: AtomicUsize = AtomicUsize::new(0usize);

#[allow(dead_code)]
fn update_modulus(m: usize) {
    DYNAMIC_MODULUS.store(m, Ordering::Relaxed);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct DynamicModulus();

impl Modulus for DynamicModulus {
    fn modulus() -> usize {
        DYNAMIC_MODULUS.load(Ordering::Relaxed)
    }
}

#[snippet("modulo")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mod<M>(usize, std::marker::PhantomData<fn() -> M>);

#[snippet("modulo")]
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
        // |v| < modulus() が保障されている
        Mod::new((v + M::modulus() as i64) as usize)
    }
}

#[snippet("modulo")]
impl<M> std::fmt::Display for Mod<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[snippet("modulo")]
impl<T, M: Modulus> std::convert::From<T> for Mod<M>
where
    usize: std::convert::From<T>,
{
    fn from(v: T) -> Self {
        Mod::new(usize::from(v))
    }
}

#[snippet("modulo")]
impl<M: Modulus> std::str::FromStr for Mod<M> {
    type Err = <usize as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        usize::from_str(s).map(|n| Mod::new(n))
    }
}

#[snippet("modulo")]
impl<M: Modulus> std::ops::Neg for Mod<M> {
    type Output = Self;
    fn neg(self) -> Self {
        Mod::new(M::modulus() - self.0)
    }
}

#[snippet("modulo")]
impl<T, M: Modulus> std::ops::Add<T> for Mod<M>
where
    T: std::convert::Into<Mod<M>>,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self {
        Mod::new(self.0 + rhs.into().0)
    }
}

#[snippet("modulo")]
impl<T, M: Modulus> std::ops::AddAssign<T> for Mod<M>
where
    T: std::convert::Into<Mod<M>>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

#[snippet("modulo")]
impl<T, M: Modulus> std::ops::Sub<T> for Mod<M>
where
    T: std::convert::Into<Mod<M>>,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self {
        Mod::new(self.0 + M::modulus() - rhs.into().0)
    }
}

#[snippet("modulo")]
impl<T, M: Modulus> std::ops::SubAssign<T> for Mod<M>
where
    T: std::convert::Into<Mod<M>>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

#[snippet("modulo")]
impl<T, M: Modulus> std::ops::Mul<T> for Mod<M>
where
    T: std::convert::Into<Mod<M>>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Mod::new(self.0 * rhs.into().0)
    }
}

#[snippet("modulo")]
impl<T, M: Modulus> std::ops::MulAssign<T> for Mod<M>
where
    T: std::convert::Into<Mod<M>>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

#[snippet("modulo")]
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

#[snippet("modulo")]
impl<T, M: Modulus> std::ops::DivAssign<T> for Mod<M>
where
    T: std::convert::Into<Mod<M>>,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

#[snippet("modulo")]
impl<M: Modulus> std::iter::Product for Mod<M> {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Mod::one(), |p, a| p * a)
    }
}

#[snippet("modulo")]
impl<M: Modulus> std::iter::Sum for Mod<M> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Mod::zero(), |p, a| p + a)
    }
}

#[snippet("modulo")]
impl<M: Modulus> num::Zero for Mod<M> {
    fn zero() -> Self {
        Mod::new(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

#[snippet("modulo")]
impl<M: Modulus> num::One for Mod<M> {
    fn one() -> Self {
        Mod::new(1)
    }

    fn is_one(&self) -> bool {
        self.0 == 1
    }
}

#[snippet("modulo_fact")]
#[allow(dead_code)]
fn generate_fact<M: Modulus>(n: usize) -> (Vec<Mod<M>>, Vec<Mod<M>>, Vec<Mod<M>>) {
    let fact: Vec<_> = std::iter::once(Mod::one())
        .chain((1..=n).scan(Mod::one(), |f, i| {
            *f = *f * i;
            Some(*f)
        }))
        .collect();
    let inv = (2..=n).fold(vec![Mod::one(), Mod::one()], |mut inv, i| {
        inv.push(-Mod::new(M::modulus() / i) * inv[M::modulus() % i]);
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
    (fact, inv, inv_fact)
}
