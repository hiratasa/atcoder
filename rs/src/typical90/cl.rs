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

#[allow(dead_code)]
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
        #[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
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
define_static_mod!(469762049, Modulus469762049, Mod469762049);
define_static_mod!(998244353, Modulus998244353, Mod998244353);
define_static_mod!(1000000007, Modulus1000000007, Mod1000000007);
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Mod<M>(usize, std::marker::PhantomData<fn() -> M>);
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

fn fft<
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + num::One,
>(
    f: &mut Vec<T>,
    es: &[T],
) {
    let n = f.len();

    if n == 1 {
        return;
    }

    assert!(n.is_power_of_two());

    let d = n.trailing_zeros() as usize;

    for i in 0..n {
        let j = i.reverse_bits() >> (std::mem::size_of::<usize>() * 8 - d);

        if i < j {
            f.swap(i, j);
        }
    }

    for i in 0..d {
        let b = 1 << i;
        let c = n >> (i + 1); // b * c == n/2

        for j in 0..c {
            for k in 0..b {
                let p = es[k * c];
                let t1 = f[j * 2 * b + k];
                let t2 = f[j * 2 * b + k + b];
                f[j * 2 * b + k] = t1 + p * t2;
                f[j * 2 * b + k + b] = t1 - p * t2;
            }
        }
    }
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
fn fft_mod<M: Modulus>(f: &mut Vec<Mod<M>>) {
    let n = f.len();
    assert!(n.is_power_of_two());
    assert!((M::modulus() - 1) % n == 0);
    let g = primitive_root(M::modulus());
    let c = pow_mod(g, (M::modulus() - 1) / n, M::modulus());
    fft(
        f,
        &(0..n)
            .scan(Mod::one(), |p, _| Some(replace(p, *p * c)))
            .collect::<Vec<_>>(),
    );
}

#[allow(dead_code)]
fn inv_fft_mod<M: Modulus>(f: &mut Vec<Mod<M>>) {
    let n = f.len();
    assert!(n.is_power_of_two());
    assert!((M::modulus() - 1) % n == 0);
    let g = primitive_root(M::modulus());
    // let c = pow_mod(g, (modulus() - 1) / n, modulus()).inv();
    let c = pow_mod(g, (M::modulus() - 1) / n * (n - 1), M::modulus());
    fft(
        f,
        &(0..n)
            .scan(Mod::one(), |p, _| Some(replace(p, *p * c)))
            .collect::<Vec<_>>(),
    );
    for x in f {
        *x /= n;
    }
}

#[allow(dead_code)]
fn convolution_mod<M: Modulus>(p: &[Mod<M>], q: &[Mod<M>]) -> Vec<Mod<M>> {
    let n0 = p.len();
    let n1 = q.len();

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

    fft_mod(&mut pf);
    fft_mod(&mut qf);

    for (x, y) in pf.iter_mut().zip(&qf) {
        *x *= *y;
    }

    inv_fft_mod(&mut pf);

    pf.resize(n0 + n1 - 1, Mod::new(0));
    pf
}

trait Convolution: std::marker::Sized {
    fn convolution(v: &[Self], u: &[Self]) -> Vec<Self>;
}

impl<M: Modulus> Convolution for Mod<M> {
    fn convolution(v: &[Self], u: &[Self]) -> Vec<Self> {
        convolution_mod(v, u)
    }
}

trait Inverse {
    fn inv(&self) -> Self;
}

impl<M: Modulus> Inverse for Mod<M> {
    fn inv(&self) -> Self {
        Mod::inv(*self)
    }
}

#[derive(Debug, Clone)]
struct FormalPowerSeries<T>(Vec<T>);

#[allow(dead_code)]
impl<T> FormalPowerSeries<T> {
    fn deg(&self) -> usize {
        self.0.len() - 1
    }

    fn shrink(&mut self, n: usize) {
        if n < self.0.len() {
            self.0.resize_with(n, || unreachable!());
        }
    }

    // apply x => -x
    fn apply_neg_x(&self) -> FormalPowerSeries<T>
    where
        T: std::ops::Neg<Output = T> + Copy,
    {
        FormalPowerSeries(
            self.0
                .iter()
                .enumerate()
                .map(|(i, &x)| if i % 2 == 0 { x } else { -x })
                .collect(),
        )
    }

    fn even(&self) -> FormalPowerSeries<T>
    where
        T: Copy,
    {
        FormalPowerSeries(self.0.iter().step_by(2).copied().collect())
    }

    fn odd(&self) -> FormalPowerSeries<T>
    where
        T: Copy,
    {
        FormalPowerSeries(self.0.iter().skip(1).step_by(2).copied().collect())
    }

    fn reverse(&mut self) {
        self.0.reverse();
    }

    fn reversed(&self) -> Self
    where
        T: Clone,
    {
        let mut r = self.clone();
        r.reverse();
        r
    }

    // mod x^n での逆元を求める
    fn inv(&self, n: usize) -> FormalPowerSeries<T>
    where
        T: Copy
            + Inverse
            + Convolution
            + std::ops::Add<T, Output = T>
            + std::ops::Mul<u32, Output = T>
            + std::ops::Neg<Output = T>,
    {
        let mut g = vec![self.0[0].inv()];
        if self.0.len() == 1 {
            return FormalPowerSeries(g);
        }

        while g.len() < n {
            let k = 2 * g.len();
            let mut h = T::convolution(&g, &g);
            h = T::convolution(&self.0[..min(self.0.len(), k)], &h);
            h.resize_with(k, || unreachable!());

            h.iter_mut().for_each(|x| *x = -*x);
            h.iter_mut().zip(g).for_each(|(x, y)| *x = *x + y * 2);

            g = h;
        }

        g.resize_with(n, || unreachable!());
        FormalPowerSeries(g)
    }
}

impl<T: Copy + std::ops::Add<Output = T>> std::ops::Add for &FormalPowerSeries<T> {
    type Output = FormalPowerSeries<T>;

    fn add(self, rhs: Self) -> Self::Output {
        FormalPowerSeries(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(x, y)| *x + *y)
                .collect(),
        )
    }
}

impl<T: Copy + std::ops::Sub<Output = T>> std::ops::Sub for &FormalPowerSeries<T> {
    type Output = FormalPowerSeries<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        FormalPowerSeries(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(x, y)| *x - *y)
                .collect(),
        )
    }
}

impl<T: Convolution> std::ops::Mul for &FormalPowerSeries<T> {
    type Output = FormalPowerSeries<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        FormalPowerSeries(T::convolution(&self.0, &rhs.0))
    }
}

// 多項式除算
// O(n logn) (nはselfの次数)
// これは形式的冪級数ではなく多項式クラスに定義すべき？
impl<
    T: Copy
        + Default
        + Inverse
        + Convolution
        + std::ops::Add<T, Output = T>
        + std::ops::Mul<u32, Output = T>
        + std::ops::Neg<Output = T>,
> std::ops::Div for &FormalPowerSeries<T>
{
    type Output = FormalPowerSeries<T>;

    fn div(self, rhs: Self) -> Self::Output {
        let n = self.deg();
        let m = rhs.deg();

        if n < m {
            return FormalPowerSeries(vec![T::default()]);
        }

        // 商の次数
        let k = n - m;

        let rev_rhs = rhs.reversed();
        let x = rev_rhs.inv(k + 1);

        let mut q = &self.reversed() * &x;
        q.shrink(k + 1);
        q.reverse();

        q
    }
}

// 隣接k+1項間漸化式で定められる数列{a_n}のn項目を計算する
// a_{k} = sum[i=0 to k-1] c_{i} * a_{k-i-1}
// https://qiita.com/ryuhe1/items/da5acbcce4ac1911f47a
// http://q.c.titech.ac.jp/docs/progs/polynomial_division.html
fn bostan_mori<M: Modulus + Clone + Default>(a: &[Mod<M>], c: &[Mod<M>], n: usize) -> Mod<M> {
    let k = c.len();

    let q = FormalPowerSeries(once(Mod::one()).chain(c.citer().map(|x| -x)).collect());

    // a.len() < k のとき、残りの項は負の番号の項をゼロとして漸化式で定められるとみなす
    let p = if a.len() < k {
        let q2 = FormalPowerSeries(
            once(Mod::one())
                .chain(c.citer().map(|x| -x))
                .take(a.len())
                .collect(),
        );
        let mut p = &FormalPowerSeries(a.to_vec()) * &q2;
        p.shrink(a.len());
        p
    } else {
        let g = FormalPowerSeries(a[0..k].to_vec());
        let mut p = &g * &q;
        p.shrink(k);
        p
    };

    successors(Some((n, p, q)), |(m, pp, qq)| {
        if *m == 0 {
            return None;
        }

        let qq2 = qq.apply_neg_x();
        let u = pp * &qq2;

        Some((
            m / 2,
            if m % 2 == 0 { u.even() } else { u.odd() },
            (qq * &qq2).even(),
        ))
    })
    .last()
    .map(|(_, pp, qq)| pp.0[0] / qq.0[0])
    .unwrap()
}

fn main() {
    type Mod = Mod998244353;

    let (n, k) = read_tuple!(usize, usize);

    let mut dp = vec![vec![]; k + 1];
    for m in (1..=k).rev() {
        if m == k {
            dp[m] = vec![Mod::one(), Mod::one()];
            continue;
        }

        let l = k / m;

        // dp[m] は、 1-x * G_{m+1} で定義される漸化式を初項1で満たす列の2項目以降
        // => 母関数 G_m は 1/(1-x*G_{m+1}(x)) の先頭落としたもの
        let q = FormalPowerSeries(
            once(Mod::one())
                .chain(dp[m + 1].citer().map(|x| -x))
                .collect(),
        );
        let mut g = q.inv(l + 2);
        g.0.remove(0);
        g.shrink(l + 1);

        dp[m] = g.0;
    }

    assert!(dp[1].len() == k + 1);
    let ans = bostan_mori(&vec![Mod::one()], &dp[1], n + 1);
    println!("{}", ans);
}
