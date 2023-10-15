#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64;
#[allow(unused_imports)]
use std::i64;
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
use rand::distributions::Standard;
use rand::prelude::Distribution;
#[allow(unused_imports)]
use rand::{rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom, Rng, SeedableRng};
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;

#[allow(unused_imports)]
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
#[allow(unused_imports)]
use proconio::source::{Readable, Source};

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
    };
    ($($x:expr),+,) => {
        it![$($x),+]
    };
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

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

use easy_ext::ext;

#[ext(IterCopyExt)]
impl<'a, I, T> I
where
    Self: IntoIterator<Item = &'a T>,
    T: 'a + Copy,
{
    fn citer(self) -> std::iter::Copied<Self::IntoIter> {
        self.into_iter().copied()
    }
}

enum Digits {}

impl Readable for Digits {
    type Output = Vec<usize>;
    fn read<R: std::io::BufRead, S: Source<R>>(source: &mut S) -> Vec<usize> {
        source
            .next_token_unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
    }
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
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

impl<M: Modulus> Distribution<Mod<M>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mod<M> {
        Mod::new(rng.gen_range(0..M::modulus()))
    }
}

trait MatrixElement:
    Copy
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Sub<Output = Self>
    + std::ops::SubAssign
    + std::ops::Mul<Output = Self>
    + std::ops::MulAssign
    + std::ops::Div<Output = Self>
    + std::ops::DivAssign
    + std::ops::Neg<Output = Self>
    + num::Zero
    + num::One
{
}

impl<T> MatrixElement for T where
    T: Copy
        + std::ops::Add<Output = Self>
        + std::ops::AddAssign
        + std::ops::Sub<Output = Self>
        + std::ops::SubAssign
        + std::ops::Mul<Output = Self>
        + std::ops::MulAssign
        + std::ops::Div<Output = Self>
        + std::ops::DivAssign
        + std::ops::Neg<Output = Self>
        + num::Zero
        + num::One
{
}

#[allow(dead_code)]
fn calc_det<T>(a: &[Vec<T>]) -> T
where
    T: MatrixElement,
{
    let mut a = a.to_vec();

    let n = a.len();

    let mut det = T::one();
    for i in 0..n {
        let idx = match (i..n).find(|&idx| !a[idx][i].is_zero()) {
            Some(idx) => idx,
            _ => return T::zero(),
        };

        if i != idx {
            a.swap(i, idx);
            det *= -T::one();
        }

        det *= a[i][i];
        let c = T::one() / a[i][i];

        for j in i..n {
            a[i][j] *= c;
        }

        for i2 in i + 1..n {
            let c = -a[i2][i];

            for j in i..n {
                a[i2][j] = a[i2][j] + c * a[i][j];
            }
        }
    }

    det
}

// PAP^(-1)の形の変換でHessenberg行列に変換
// O(N^3)
#[allow(dead_code)]
fn to_upper_hessenberg_matrix<T>(a: &mut [Vec<T>])
where
    T: MatrixElement,
{
    let n = a.len();

    for idx in 1..n {
        let idx2 = match (idx..n).find(|&i| !a[i][idx - 1].is_zero()) {
            Some(idx2) => idx2,
            _ => continue,
        };

        a.swap(idx, idx2);
        a.iter_mut().for_each(|row| {
            row.swap(idx, idx2);
        });

        let inv = T::one() / a[idx][idx - 1];
        for i in idx + 1..n {
            let c = -a[i][idx - 1] * inv;

            // 行idxのc倍を行iに足す
            for j in idx - 1..n {
                a[i][j] = a[i][j] + c * a[idx][j];
            }

            // 列iのc倍を列idxから引く
            for i2 in 0..n {
                a[i2][idx] = a[i2][idx] - c * a[i2][i];
            }
        }
    }
}

// det(A-xI)
// O(N^3)
#[allow(dead_code)]
fn characteristic_polynomial<T>(a: &[Vec<T>]) -> Vec<T>
where
    T: MatrixElement,
{
    let mut a = a.to_vec();

    to_upper_hessenberg_matrix(&mut a);

    let n = a.len();

    // 左上のi行i列部分の行列式(多項式)
    let mut dets = vec![vec![]; n + 1];
    dets[0] = vec![T::one()];
    for i in 0..n {
        dets[i + 1] = vec![T::zero(); i + 2];

        // (i,i)要素 * 残り
        for deg in 0..=i {
            dets[i + 1][deg] = dets[i + 1][deg] + a[i][i] * dets[i][deg];
        }
        for deg in 0..=i {
            dets[i + 1][deg + 1] = dets[i + 1][deg + 1] - dets[i][deg];
        }

        let mut p = T::one();
        for i2 in (0..i).rev() {
            // (i2+1,i2)要素 * ...
            p *= -a[i2 + 1][i2];

            // (i2,i)要素 * 残り
            for deg in 0..=i2 {
                dets[i + 1][deg] = dets[i + 1][deg] + p * a[i2][i] * dets[i2][deg];
            }
        }
    }

    std::mem::take(&mut dets[n])
}

// det(A+xB)をdet(A'+xI)/(d*x^k)の形に変換し、(d, k)を返す
// 恒等的に0の場合はNoneを返す
// O(N^3)
#[allow(dead_code)]
fn xb_to_xidentity<T>(a: &mut [Vec<T>], b: &mut [Vec<T>]) -> Option<(T, usize)>
where
    T: MatrixElement,
{
    let n = a.len();
    assert!(b.len() == n);

    let mut det = T::one();
    let mut deg = 0;
    for idx in 0..n {
        let idx2 = if let Some(idx2) = (idx..n).find(|&i| !b[i][idx].is_zero()) {
            idx2
        } else {
            // Bのidx番目の列が全て0なので、A+xB全体のidx番目の列にxを掛け、Aから項を持ってくる
            loop {
                deg += 1;
                if deg > n {
                    return None;
                }
                for i in 0..n {
                    assert!(b[i][idx].is_zero());
                    b[i][idx] = a[i][idx];
                    a[i][idx] = T::zero();
                }

                // idx行目より前を再度履き出す
                for idx2 in 0..idx {
                    let c = -b[idx2][idx];
                    for i in 0..n {
                        a[i][idx] = a[i][idx] + a[i][idx2] * c;
                        b[i][idx] = b[i][idx] + b[i][idx2] * c;
                    }
                }

                // 改めて非零の行を探す
                if let Some(idx2) = (idx..n).find(|&i| !b[i][idx].is_zero()) {
                    break idx2;
                }
                // 非零の行がなかった場合でも、再度の履き出しによりAのidx列目に0以外の値が出現している場合があるので、再度同様の手順を繰り返す
            }
        };
        if idx != idx2 {
            a.swap(idx, idx2);
            b.swap(idx, idx2);
            det *= -T::one();
        }

        let c = T::one() / b[idx][idx];
        det *= c;
        for j in 0..n {
            a[idx][j] *= c;
            b[idx][j] *= c;
        }

        for i in idx + 1..n {
            let c = -b[i][idx];
            for j in 0..n {
                a[i][j] = a[i][j] + a[idx][j] * c;
                b[i][j] = b[i][j] + b[idx][j] * c;
            }
        }

        for j in idx + 1..n {
            let c = -b[idx][j];
            for i in 0..n {
                a[i][j] = a[i][j] + a[i][idx] * c;
                b[i][j] = b[i][j] + b[i][idx] * c;
            }
        }
    }

    Some((det, deg))
}

// det(A+xB)
// O(N^3)
#[allow(dead_code)]
fn calc_det_a_xb<T>(a: &[Vec<T>], b: &[Vec<T>]) -> Vec<T>
where
    T: MatrixElement,
{
    let n = a.len();
    assert!(b.len() == n);

    let mut a = a.to_vec();
    let mut b = b.to_vec();

    // 適当な変形でdet(A'+xI)/(d*x^k) の形にする
    let Some((d, k)) = xb_to_xidentity(&mut a, &mut b) else {
        return vec![T::zero()];
    };

    // det(A'-xI)
    let mut poly = characteristic_polynomial(&a);

    // det(A'+xI)
    poly.iter_mut()
        .skip(1)
        .step_by(2)
        .for_each(|v| *v *= -T::one());

    // det(A+xB)
    poly.iter_mut().for_each(|v| {
        *v = *v / d;
    });
    poly.drain(0..k);

    poly
}

fn main() {
    type Mod = Mod998244353;

    input! {
        n: usize,
        p: [Usize1; n]
    }

    let inverts = p
        .citer()
        .enumerate()
        .tuple_combinations()
        .filter(|&((_, v), (_, u))| v > u)
        .fold(vec![vec![false; n]; n], |mut inverts, ((i, _), (j, _))| {
            inverts[i][j] = true;
            inverts[j][i] = true;
            inverts
        });

    // 反転している箇所の重みをx, そうでない箇所の重みを1としたラプラシアン行列、の1行目・1列目を除いたもの
    // A + xB
    let a = (1..n)
        .map(|i| {
            (1..n)
                .map(|j| {
                    if i == j {
                        let k = inverts[i].citer().filter(|&x| x).count();
                        Mod::new(n - 1 - k)
                    } else if inverts[i][j] {
                        Mod::zero()
                    } else {
                        -Mod::one()
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let b = (1..n)
        .map(|i| {
            (1..n)
                .map(|j| {
                    if i == j {
                        let k = inverts[i].citer().filter(|&x| x).count();
                        Mod::new(k)
                    } else if inverts[i][j] {
                        -Mod::one()
                    } else {
                        Mod::zero()
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let det = calc_det_a_xb(&a, &b);
    println!(
        "{}",
        det.citer().chain(repeat(Mod::zero())).take(n).join(" ")
    );
}
