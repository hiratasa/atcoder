use std::{collections::VecDeque, mem::replace};

use proconio::input;

fn main() {
    type Mod = Mod998244353;
    input! {
        n: usize,
        a: [usize; n],
    };

    let s = a.iter().sum::<usize>();
    let mut q = a
        .iter()
        .map(|&x| vec![Mod::one(), Mod::new(x)])
        .collect::<VecDeque<_>>();
    while q.len() > 1 {
        let x = q.pop_front().unwrap();
        let y = q.pop_front().unwrap();

        let z = convolution_mod(&x, &y);
        q.push_back(z);
    }

    let z = q.pop_front().unwrap();

    let f = (1..)
        .scan(Mod::one(), |f, x| Some(replace(f, *f * x)))
        .take(n + 1)
        .collect::<Vec<_>>();
    let g = (1..=s)
        .rev()
        .take(n + 1)
        .scan(Mod::one(), |f, x| Some(replace(f, *f * x)))
        .collect::<Vec<_>>();

    let ans = 1 + (1..=n).map(|i| z[i] * f[i] / g[i]).sum::<Mod>();

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

trait RootSupplier<'a, T> {
    fn w_pow(&self, i: usize) -> T;
    fn iw_pow(&self, i: usize) -> T;
}
#[allow(dead_code)]
trait Butterfly<T>
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>,
{
    type RootSupplier<'a>: RootSupplier<'a, T>
    where
        Self: 'a;
    fn get_roots<'a>(&'a self, h: usize) -> Self::RootSupplier<'a>;
    fn butterfly(&self, f: &mut [T]) {
        let n = f.len();
        assert!(n.is_power_of_two());
        let h = n.trailing_zeros() as usize;
        let roots = self.get_roots(h);
        for i in 0..=h {
            let b = 1 << i;
            let c = n >> (i + 1);
            let d = n >> i;
            for k in 0..b {
                for j in 0..c {
                    let p = roots.w_pow(j * b);
                    let t0 = f[k * d + j];
                    let t1 = f[k * d + j + c];
                    f[k * d + j] = t0 + t1;
                    f[k * d + j + c] = p * (t0 - t1);
                }
            }
        }
    }
    fn butterfly_inv(&self, f: &mut [T]) {
        let n = f.len();
        assert!(n.is_power_of_two());
        let h = n.trailing_zeros() as usize;
        let roots = self.get_roots(h);
        for i in 0..=h {
            let b = 1 << i;
            let c = n >> (i + 1);
            let b2 = b * 2;
            for j in 0..c {
                for k in 0..b {
                    let p = roots.iw_pow(k * c);
                    let t1 = f[j * b2 + k];
                    let t2 = p * f[j * b2 + k + b];
                    f[j * b2 + k] = t1 + t2;
                    f[j * b2 + k + b] = t1 - t2;
                }
            }
        }
    }
}
#[derive(Debug)]
struct ModRootSupplier<'a, M> {
    w_pows: std::cell::Ref<'a, (Vec<Mod<M>>, Vec<Mod<M>>)>,
}
impl<'a, M> ModRootSupplier<'a, M> {
    fn new(w_pows: std::cell::Ref<'a, (Vec<Mod<M>>, Vec<Mod<M>>)>) -> ModRootSupplier<'a, M> {
        ModRootSupplier { w_pows }
    }
}
impl<'a, M> RootSupplier<'a, Mod<M>> for ModRootSupplier<'a, M>
where
    M: Copy,
{
    fn w_pow(&self, i: usize) -> Mod<M> {
        self.w_pows.0[i]
    }
    fn iw_pow(&self, i: usize) -> Mod<M> {
        self.w_pows.1[i]
    }
}
#[derive(Debug, Clone)]
struct ModButterfly<M> {
    root: Mod<M>,
    w_pows: std::cell::RefCell<Vec<(Vec<Mod<M>>, Vec<Mod<M>>)>>,
}
#[allow(dead_code)]
impl<M> ModButterfly<Mod<M>>
where
    M: Modulus,
{
    fn new() -> ModButterfly<M> {
        ModButterfly {
            root: Mod::new(primitive_root(M::modulus())),
            w_pows: std::cell::RefCell::new(vec![]),
        }
    }
}
impl<M> Butterfly<Mod<M>> for ModButterfly<M>
where
    M: Modulus,
{
    type RootSupplier < 'a > = ModRootSupplier < 'a , M > where M : 'a ;
    fn get_roots<'a>(&'a self, h: usize) -> ModRootSupplier<'a, M> {
        let mut w_pows = self.w_pows.borrow_mut();
        if h >= w_pows.len() {
            w_pows.resize(h + 1, (vec![], vec![]))
        }
        if w_pows[h].0.is_empty() {
            let m = 1 << h;
            let c = pow_mod(self.root.0, (M::modulus() - 1) / m, M::modulus());
            w_pows[h].0 = (0..m)
                .scan(Mod::<M>::new(1), |p, _| Some(std::mem::replace(p, *p * c)))
                .collect::<Vec<_>>();
            let cinv = pow_mod(self.root.0, (M::modulus() - 1) / m * (m - 1), M::modulus());
            w_pows[h].1 = (0..m)
                .scan(Mod::<M>::new(1), |p, _| {
                    Some(std::mem::replace(p, *p * cinv))
                })
                .collect::<Vec<_>>();
        }
        drop(w_pows);
        let w_pows = self.w_pows.borrow();
        ModRootSupplier::new(std::cell::Ref::map(w_pows, |w_pows| &w_pows[h]))
    }
}
#[allow(dead_code)]
fn convolution_impl<
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::MulAssign
        + std::ops::DivAssign,
>(
    p: &mut [T],
    q: &mut [T],
    butterfly: &impl Butterfly<T>,
    n_as_t: T,
) {
    let n = p.len();
    assert!(q.len() == n);
    assert!(n.is_power_of_two());
    butterfly.butterfly(p);
    butterfly.butterfly(q);
    for (x, y) in p.iter_mut().zip(q) {
        *x *= *y;
    }
    butterfly.butterfly_inv(p);
    p.iter_mut().for_each(|x| *x /= n_as_t);
}
#[allow(dead_code)]
fn primitive_root(m: usize) -> usize {
    match m {
        2 => return 1,
        167772161 => return 3,
        469762049 => return 3,
        754974721 => return 11,
        998244353 => return 3,
        1224736769 => return 3,
        1811939329 => return 13,
        2013265921 => return 31,
        _ => {}
    };
    let primes = (2..)
        .try_fold((vec![], m - 1), |(mut primes, x), i| {
            if i * i > x {
                if x > 1 {
                    primes.push(x);
                }
                Err(primes)
            } else if x % i > 0 {
                Ok((primes, x))
            } else {
                primes.push(i);
                let x = itertools::iterate(x, |x| x / i)
                    .find(|&x| x % i > 0)
                    .unwrap();
                Ok((primes, x))
            }
        })
        .unwrap_err();
    (2..)
        .find(|&g| primes.iter().all(|&p| pow_mod(g, (m - 1) / p, m) != 1))
        .unwrap()
}
#[allow(dead_code)]
pub fn convolution_mod<M: Modulus>(p: &[Mod<M>], q: &[Mod<M>]) -> Vec<Mod<M>> {
    let n0 = p.len();
    let n1 = q.len();
    if std::cmp::min(n0, n1) <= 64 {
        let mut r = vec![Mod::new(0); n0 + n1 - 1];
        for (i, &pp) in p.iter().enumerate() {
            for (j, &qq) in q.iter().enumerate() {
                r[i + j] = r[i + j] + pp * qq;
            }
        }
        return r;
    }
    let n = (n0 + n1 - 1).next_power_of_two();
    let mut pf = p
        .iter()
        .copied()
        .chain(std::iter::repeat(Mod::new(0)))
        .take(n)
        .collect::<Vec<_>>();
    let mut qf = q
        .iter()
        .copied()
        .chain(std::iter::repeat(Mod::new(0)))
        .take(n)
        .collect::<Vec<_>>();
    let butterfly = ModButterfly::new();
    convolution_impl(&mut pf, &mut qf, &butterfly, Mod::new(n));
    pf.resize(n0 + n1 - 1, Mod::new(0));
    pf
}
