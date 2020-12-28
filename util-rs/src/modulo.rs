use cargo_snippet::snippet;

#[snippet("modulo")]
use num::{One, Zero};

// for dynamic modulus
// use std::sync::atomic::{AtomicUsize, Ordering};

// static modulus_impl: AtomicUsize = AtomicUsize::new(0usize);

// fn update_modulus(m: usize) {
//     modulus_impl.store(m, Ordering::Relaxed);
// }

// fn modulus() -> usize {
//     modulus_impl.load(Ordering::Relaxed)
// }

#[snippet("modulo")]
fn modulus() -> usize {
    1_000_000_007
}

#[snippet("modulo")]
#[derive(Clone, Copy, Debug)]
struct Mod(usize);

#[snippet("modulo")]
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

            if p % 2 == 0 {
                r * r
            } else {
                r * r * self
            }
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
        // |v| < modulus() が保障されている
        Mod::new((v + modulus() as i64) as usize)
    }
}

#[snippet("modulo")]
impl std::fmt::Display for Mod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[snippet("modulo")]
impl<T> std::convert::From<T> for Mod
where
    usize: std::convert::From<T>,
{
    fn from(v: T) -> Mod {
        Mod::new(usize::from(v))
    }
}

#[snippet("modulo")]
impl std::str::FromStr for Mod {
    type Err = <usize as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        usize::from_str(s).map(|n| Mod::new(n))
    }
}

#[snippet("modulo")]
impl std::ops::Neg for Mod {
    type Output = Self;
    fn neg(self) -> Self {
        Mod::new(modulus() - self.0)
    }
}

#[snippet("modulo")]
impl<T> std::ops::Add<T> for Mod
where
    T: std::convert::Into<Mod>,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self {
        Mod::new(self.0 + rhs.into().0)
    }
}

#[snippet("modulo")]
impl<T> std::ops::AddAssign<T> for Mod
where
    T: std::convert::Into<Mod>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

#[snippet("modulo")]
impl<T> std::ops::Sub<T> for Mod
where
    T: std::convert::Into<Mod>,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self {
        Mod::new(self.0 + modulus() - rhs.into().0)
    }
}

#[snippet("modulo")]
impl<T> std::ops::SubAssign<T> for Mod
where
    T: std::convert::Into<Mod>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

#[snippet("modulo")]
impl<T> std::ops::Mul<T> for Mod
where
    T: std::convert::Into<Mod>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Mod::new(self.0 * rhs.into().0)
    }
}

#[snippet("modulo")]
impl<T> std::ops::MulAssign<T> for Mod
where
    T: std::convert::Into<Mod>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

#[snippet("modulo")]
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

#[snippet("modulo")]
impl<T> std::ops::DivAssign<T> for Mod
where
    T: std::convert::Into<Mod>,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

#[snippet("modulo")]
impl std::iter::Product for Mod {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Mod::one(), |p, a| p * a)
    }
}

#[snippet("modulo")]
impl std::iter::Sum for Mod {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Mod::zero(), |p, a| p + a)
    }
}

#[snippet("modulo")]
impl num::Zero for Mod {
    fn zero() -> Self {
        Mod::new(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

#[snippet("modulo")]
impl num::One for Mod {
    fn one() -> Self {
        Mod::new(1)
    }

    fn is_one(&self) -> bool {
        self.0 == 1
    }
}

#[snippet("modulo_fact")]
#[allow(dead_code)]
fn generate_fact(n: usize) {
    let fact: Vec<_> = std::iter::once(Mod::one())
        .chain((1..=n).scan(Mod::one(), |f, i| {
            *f = *f * i;
            Some(*f)
        }))
        .collect();
    let inv = (2..=n).fold(vec![Mod::one(), Mod::one()], |mut inv, i| {
        inv.push(-Mod::new(modulus() / i) * inv[modulus() % i]);
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
