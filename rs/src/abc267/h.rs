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
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
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

#[allow(dead_code)]
fn println_opt<T: Copy + std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
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
    T: num::traits::Unsigned,
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
impl<M: Modulus> std::ops::Add<Mod<M>> for Mod<M> {
    type Output = Self;
    fn add(self, rhs: Mod<M>) -> Self {
        Mod::new(self.0 + rhs.0)
    }
}
impl<M: Modulus> std::ops::Add<usize> for Mod<M> {
    type Output = Self;
    fn add(self, rhs: usize) -> Self {
        Mod::new(self.0 + rhs)
    }
}
impl<M: Modulus> std::ops::Add<Mod<M>> for usize {
    type Output = Mod<M>;
    fn add(self, rhs: Mod<M>) -> Mod<M> {
        Mod::new(self + rhs.0)
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
        Mod::new(self.0 + M::modulus() - rhs.0)
    }
}
impl<M: Modulus> std::ops::Sub<usize> for Mod<M> {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self {
        Mod::new(self.0 + M::modulus() - rhs % M::modulus())
    }
}
impl<M: Modulus> std::ops::Sub<Mod<M>> for usize {
    type Output = Mod<M>;
    fn sub(self, rhs: Mod<M>) -> Mod<M> {
        Mod::new(self + M::modulus() - rhs.0)
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

// 周波数間引きバタフライ演算
// w_pow[1]^n = 1
// w_pow[i] = w_pow[1]^i
#[allow(dead_code)]
fn butterfly<
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>,
>(
    f: &mut [T],
    w_pow: &[T],
) {
    let n = f.len();

    assert!(n.is_power_of_two());

    let h = n.trailing_zeros() as usize;
    let w4 = w_pow[n / 4];

    // i回目の演算開始時点で、2^(h-i)で割った余りで等しい要素からなる長さ2^iの列が変換済み
    // i回目の演算では、2^(h-i-1)離れて隣接する項同士を足し引きして列の長さを2倍にする
    // (もしくは2回分まとめて4倍にする)
    for (i, step) in (0..=h + 1)
        .step_by(2)
        .map(|i| usize::min(i, h))
        .tuple_windows()
        .map(|(i, i2)| (i, i2 - i))
    {
        if step == 1 {
            // 変換済みの列長
            let b = 1 << i;
            let c = n >> (i + 1);
            let d = n >> i; // b * d == n

            for k in 0..b {
                for j in 0..c {
                    let p = w_pow[j * b];
                    let t0 = f[k * d + j];
                    let t1 = f[k * d + j + c];
                    f[k * d + j] = t0 + t1;
                    f[k * d + j + c] = p * (t0 - t1);
                }
            }
        } else {
            assert!(step == 2);

            // 変換済みの列長
            let b = 1 << i;
            let c = n >> (i + 2);
            let d = n >> i; // b * d == n

            for k in 0..b {
                for j in 0..c {
                    let p = w_pow[j * b];
                    let p2 = p * p;
                    let p3 = p2 * p;
                    let t0 = f[k * d + j];
                    let t1 = f[k * d + j + c];
                    let t2 = f[k * d + j + 2 * c];
                    let t3 = f[k * d + j + 3 * c];
                    f[k * d + j] = t0 + t1 + t2 + t3;
                    f[k * d + j + c] = p2 * (t0 - t1 + t2 - t3);
                    f[k * d + j + 2 * c] = p * (t0 + w4 * t1 - t2 - w4 * t3);
                    f[k * d + j + 3 * c] = p3 * (t0 - w4 * t1 - t2 + w4 * t3);
                }
            }
        }
    }
}

// 時間間引きバタフライ演算
// w_pow[1]^n = 1
// w_pow[i] = w_pow[1]^i
#[allow(dead_code)]
fn butterfly_inv<
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>,
>(
    f: &mut [T],
    w_pow: &[T],
) {
    let n = f.len();

    assert!(n.is_power_of_two());

    let h = n.trailing_zeros() as usize;
    let w4 = w_pow[n / 4];

    // i回目の演算開始時点で、各長さ2^iのブロックが変換済み
    // i回目の演算では、隣接するブロックの対応する項同士を足し引きして変換済みのブロック長を2倍にする
    // (もしくは2回分まとめて4倍にする)
    for (i, step) in (0..=h + 1)
        .step_by(2)
        .map(|i| usize::min(i, h))
        .tuple_windows()
        .map(|(i, i2)| (i, i2 - i))
    {
        if step == 1 {
            // 変換済みのブロック長
            let b = 1 << i;
            let c = n >> (i + 1); // (2 * b) * c == n
            let b2 = b * 2;

            for j in 0..c {
                for k in 0..b {
                    let p = w_pow[k * c];
                    let t1 = f[j * b2 + k];
                    let t2 = p * f[j * b2 + k + b];
                    f[j * b2 + k] = t1 + t2;
                    f[j * b2 + k + b] = t1 - t2;
                }
            }
        } else {
            assert!(step == 2);

            // 変換済みのブロック長
            let b = 1 << i;
            let c = n >> (i + 2); // (4 * b) * c == n
            let b4 = 4 * b;

            for j in 0..c {
                for k in 0..b {
                    let p = w_pow[k * c];
                    let p2 = p * p;
                    let p3 = p2 * p;
                    let t0 = f[j * b4 + k];
                    let t1 = p2 * f[j * b4 + k + b];
                    let t2 = p * f[j * b4 + k + 2 * b];
                    let t3 = p3 * f[j * b4 + k + 3 * b];
                    f[j * b4 + k] = t0 + t1 + t2 + t3;
                    f[j * b4 + k + b] = t0 - t1 + w4 * t2 - w4 * t3;
                    f[j * b4 + k + 2 * b] = t0 + t1 - t2 - t3;
                    f[j * b4 + k + 3 * b] = t0 - t1 - w4 * t2 + w4 * t3;
                }
            }
        }
    }
}

#[allow(dead_code)]
fn reverse_bits_order<
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>,
>(
    f: &mut [T],
) {
    let n = f.len();

    assert!(n.is_power_of_two());

    let h = n.trailing_zeros() as usize;

    for i in 0..n {
        let j = i.reverse_bits() >> (std::mem::size_of::<usize>() * 8 - h);

        if i < j {
            f.swap(i, j);
        }
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
    w_pow: &[T],
    iw_pow: &[T],
    n_as_t: T,
) {
    let n = p.len();

    assert!(q.len() == n);
    assert!(n.is_power_of_two());

    butterfly(p, &w_pow);
    butterfly(q, &w_pow);

    for (x, y) in p.iter_mut().zip(q) {
        *x *= *y;
    }

    butterfly_inv(p, &iw_pow);

    p.iter_mut().for_each(|x| *x /= n_as_t);
}

// 素数mに対して、原始根を求める
// g^k != 1 (1<=k<m-1), g^(m-1) = 1
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

    // m - 1の素因数分解
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

    // naive
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

    let g = primitive_root(M::modulus());
    let c = pow_mod(g, (M::modulus() - 1) / n, M::modulus());
    let w_pow = (0..n)
        .scan(Mod::new(1), |p, _| Some(std::mem::replace(p, *p * c)))
        .collect::<Vec<_>>();
    let cinv = pow_mod(g, (M::modulus() - 1) / n * (n - 1), M::modulus());
    let iw_pow = (0..n)
        .scan(Mod::new(1), |p, _| Some(std::mem::replace(p, *p * cinv)))
        .collect::<Vec<_>>();
    convolution_impl(&mut pf, &mut qf, &w_pow, &iw_pow, Mod::new(n));

    pf.resize(n0 + n1 - 1, Mod::new(0));
    pf
}

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

fn main() {
    type Mod = Mod998244353;

    let (n, m) = read_tuple!(usize, usize);
    let a = read_row::<usize>();

    let (fact, inv, inv_fact) = generate_fact(m);

    let p = a.citer().fold(vec![0; 11], |mut p, x| {
        p[x] += 1;
        p
    });

    let combi = |x: usize, y: usize| {
        if x < y {
            Mod::zero()
        } else {
            fact[x] * inv_fact[y] * inv_fact[x - y]
        }
    };

    let v = p
        .citer()
        .enumerate()
        .map(|(i, d)| {
            let t0 = (0..=min(m, i * d))
                .map(|s| {
                    if i == 0 || s % i > 0 || s / i % 2 != 0 {
                        Mod::zero()
                    } else {
                        combi(d, s / i)
                    }
                })
                .collect::<Vec<_>>();
            let t1 = (0..=min(m, i * d))
                .map(|s| {
                    if i == 0 || s % i > 0 || s / i % 2 != 1 {
                        Mod::zero()
                    } else {
                        combi(d, s / i)
                    }
                })
                .collect::<Vec<_>>();

            [t0, t1]
        })
        .collect::<Vec<_>>();

    let t = v
        .into_iter()
        .skip(1)
        .fold1(|t, s| {
            let mut tt = [
                vec![Mod::zero(); min(m, t[0].len() + s[0].len() - 1) + 1],
                vec![Mod::zero(); min(m, t[0].len() + s[0].len() - 1) + 1],
            ];
            for i in 0..2 {
                for j in 0..2 {
                    let tmp = convolution_mod(&t[i], &s[j]);
                    izip!(tt[(i + j) % 2].iter_mut(), tmp).for_each(|(x, y)| *x += y);
                }
            }

            tt
        })
        .unwrap();

    let ans = t[1].get(m).copied().unwrap_or(Mod::zero());
    println!("{}", ans);
}
