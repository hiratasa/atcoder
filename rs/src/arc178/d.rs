use proconio::input;

fn main() {
    type Mod = Mod998244353;

    input! {
        n: usize, m: usize,
        a: [usize; m],
    };

    let mut t: Vec<Result<usize, usize>> = a.iter().copied().map(|x| Ok(x)).collect::<Vec<_>>();
    let mut dp = vec![vec![Mod::zero(); n + 1]; n + 1];
    dp[0][0] = Mod::one();
    for i in (0..n).rev() {
        if let Some(idx) = t.iter().position(|&x| x == Ok(i)) {
            t[idx] = Err(1);

            if idx == 0 {
                if 1 < t.len() {
                    if let Err(x) = t[1] {
                        t.remove(0);
                        t[0] = Err(x + 1);
                    }
                }
            } else if idx == t.len() - 1 {
                if 1 < t.len() {
                    if let Err(x) = t[idx - 1] {
                        t.remove(idx);
                        t[idx - 1] = Err(x + 1);
                    }
                }
            } else {
                match (t[idx - 1], t[idx + 1]) {
                    (Ok(_), Ok(_)) => {}
                    (Ok(_), Err(x)) => {
                        t.remove(idx);
                        t[idx] = Err(x + 1);
                    }
                    (Err(x), Ok(_)) => {
                        t.remove(idx);
                        t[idx - 1] = Err(x + 1);
                    }
                    (Err(x), Err(y)) => {
                        t.remove(idx);
                        t.remove(idx);
                        t[idx - 1] = Err(x + y + 1);
                    }
                }
            }

            let mut left = 0;
            let mut right = 0;
            if let Err(x) = t[0] {
                t.remove(0);
                left = x;
            }
            if let Some(&Err(x)) = t.last() {
                t.remove(t.len() - 1);
                right = x;
            }

            let mut next = vec![vec![Mod::zero(); n + 1]; n + 1];
            if t.is_empty() {
                for i in 0..=n {
                    for j in 0..=n {
                        if i + j + left + right > n {
                            assert!(dp[i][j].is_zero());
                            continue;
                        }
                        next[i + j + left + right][0] += dp[i][j];
                    }
                }
            } else {
                for i in 0..=n - left {
                    for j in 0..=n - right {
                        next[i + left][j + right] = dp[i][j];
                    }
                }
            }

            dp = next;
        } else if t.is_empty() {
            let s = Mod::new(2).pow(i);

            let ans = s
                * (0..=n)
                    .map(|j| {
                        dp[j][0]
                            * (1..=i + 1 + j)
                                .rev()
                                .take(j)
                                .map(|x| Mod::new(x))
                                .product::<Mod>()
                            / (1..=j).map(|x| Mod::new(x)).product::<Mod>()
                    })
                    .sum::<Mod>();

            dp = vec![vec![ans]];

            break;
        } else {
            // let mut next = vec![vec![Mod::zero(); n + 2]; n + 2];

            // for left in 0..=n {
            //     for right in 0..=n {
            // 左からj番目に置く
            // for j in 0..=left {
            //     next[left - j][right] += dp[left][right];
            // }

            // 右からj番目に置く
            // for j in 0..=right {
            //     next[left][right - j] += dp[left][right];
            // }
            //     }
            // }

            let mut next0 = vec![vec![Mod::zero(); n + 2]; n + 2];
            let mut next1 = vec![vec![Mod::zero(); n + 2]; n + 2];
            for left in 0..=n {
                for right in 0..=n {
                    // 左からj番目に置く
                    // for j in 0..=left {
                    //     next[left - j][right] += dp[left][right];
                    // }
                    next0[right][0] += dp[left][right];
                    next0[right][left + 1] -= dp[left][right];

                    // 右からj番目に置く
                    // for j in 0..=right {
                    //     next[left][right - j] += dp[left][right];
                    // }
                    next1[left][0] += dp[left][right];
                    next1[left][right + 1] -= dp[left][right];
                }
            }

            for i in 0..=n {
                for j in 0..=n {
                    next0[i][j + 1] = next0[i][j + 1] + next0[i][j];
                    next1[i][j + 1] = next1[i][j + 1] + next1[i][j];
                }
            }

            let mut next = vec![vec![Mod::zero(); n + 2]; n + 2];
            for i in 0..=n {
                for j in 0..=n {
                    next[i][j] = next0[j][i] + next1[i][j];
                }
            }

            dp = next;
        }
    }

    let ans = dp
        .into_iter()
        .map(|row| row.into_iter().sum::<Mod>())
        .sum::<Mod>();

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
        if self.0 == 0 { self } else { self * rhs.inv() }
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
impl<M: Modulus> rand::distr::Distribution<Mod<M>> for rand::distr::StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Mod<M> {
        Mod::new(rng.random_range(0..M::modulus()))
    }
}
