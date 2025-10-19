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

define_static_mod!(200003, Modulus200003, Mod200003);
define_static_mod!(469762049, Modulus469762049, Mod469762049);
define_static_mod!(998244353, Modulus998244353, Mod998244353);

#[derive(Clone, Copy, PartialEq, Eq)]
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
    usize: std::convert::From<T>,
{
    fn from(v: T) -> Self {
        Mod::new(usize::from(v))
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

// Calculate fft: g[i] = sum[j=0 to n] f[j] * c^(i * j)
// - length must be power of two
// - c^n == 1
// 計算量 O(n logn)
fn fft<
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + num::One,
>(
    f: &mut Vec<T>,
    c: T,
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

    let cs = std::iter::successors(Some(c), |cc| Some(*cc * *cc))
        .take(d)
        .collect::<Vec<_>>();
    for i in 0..d {
        let b = 1 << i;
        let c = n >> (i + 1); // b * c == n/2
        let z = cs[d - 1 - i];

        for j in 0..c {
            let mut p = T::one();

            for k in 0..b {
                let t1 = f[j * 2 * b + k];
                let t2 = f[j * 2 * b + k + b];
                f[j * 2 * b + k] = t1 + p * t2;
                f[j * 2 * b + k + b] = t1 - p * t2;
                p = p * z;
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
    fft(f, Mod::new(c));
}

#[allow(dead_code)]
fn inv_fft_mod<M: Modulus>(f: &mut Vec<Mod<M>>) {
    let n = f.len();
    assert!(n.is_power_of_two());
    assert!((M::modulus() - 1) % n == 0);
    let g = primitive_root(M::modulus());
    // let c = pow_mod(g, (modulus() - 1) / n, modulus()).inv();
    let c = pow_mod(g, (M::modulus() - 1) / n * (n - 1), M::modulus());
    fft(f, Mod::new(c));
    for x in f {
        *x /= n;
    }
}

#[allow(dead_code)]
fn convolution_mod<M: Modulus>(p: &Vec<Mod<M>>, q: &Vec<Mod<M>>) -> Vec<Mod<M>> {
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

fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (_zero, g, _u, v) = std::iter::successors(Some((a, b, 1, 0)), |&(a, b, u, v)| {
        if a == 0 {
            None
        } else {
            Some((b % a, a, -u * (b / a) + v, u))
        }
    })
    .last()
    .unwrap();

    (v, (g - a * v) / b, g)
}

fn convolution(p: &Vec<usize>, q: &Vec<usize>) -> Vec<usize> {
    define_static_mod!(469762049, Modulus469762049, Mod469762049);
    define_static_mod!(998244353, Modulus998244353, Mod998244353);

    type Mod1 = Mod469762049;
    type Mod2 = Mod998244353;

    let M1 = Mod1::modulus() as i64;
    let M2 = Mod2::modulus() as i64;

    let p1 = p.iter().copied().map(|x| Mod1::new(x)).collect();
    let q1 = q.iter().copied().map(|x| Mod1::new(x)).collect();
    let conv1 = convolution_mod(&p1, &q1);

    let p2 = p.iter().copied().map(|x| Mod2::new(x)).collect();
    let q2 = q.iter().copied().map(|x| Mod2::new(x)).collect();
    let conv2 = convolution_mod(&p2, &q2);

    // u * M1 + v * M2 = 1
    let (u, v, _1) = extgcd(M1, M2);

    conv1
        .into_iter()
        .zip(conv2)
        .map(|(r1, r2)| {
            (r1.0 as i64 * v % M1 * M2 + r2.0 as i64 * u % M2 * M1).rem_euclid(M1 * M2) as usize
        })
        .collect()
}

fn main() {
    const M: usize = 200003;

    type Mod = Mod200003;

    let n: usize = read();
    let a = read_row::<usize>();

    const Q: usize = 2;

    let pows = iterate(Mod::one(), |&q| q * Q).take(M - 1).collect_vec();
    let logs = pows
        .citer()
        .enumerate()
        .fold(vec![None; M], |mut logs, (i, pp)| {
            logs[pp.0] = Some(i);
            logs
        });

    let freq = a
        .citer()
        .filter(|&aa| aa > 0)
        .map(|aa| logs[aa].unwrap())
        .fold(vec![0; M - 1], |mut freq, bb| {
            freq[bb] += 1;
            freq
        });
    let conv = convolution(&freq, &freq);
    let sqsum = a.citer().map(|aa| aa * aa % M).sum::<usize>();

    let ans = (conv
        .citer()
        .enumerate()
        .map(|(i, c)| pows[i % (M - 1)].0 * c)
        .sum::<usize>()
        - sqsum)
        / 2;
    println!("{}", ans);
}
