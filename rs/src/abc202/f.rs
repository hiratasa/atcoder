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

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct Vector(i64, i64);

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Mul<Vector> for i64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector(self * rhs.0, self * rhs.1)
    }
}

impl Vector {
    pub fn cross(self, rhs: Self) -> i64 {
        self.0 * rhs.1 - rhs.0 * self.1
    }
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
define_static_mod!(469762049, Modulus469762049, Mod469762049);
define_static_mod!(998244353, Modulus998244353, Mod998244353);
define_static_mod!(1000000007, Modulus1000000007, Mod1000000007);
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

fn gcd(x: i64, y: i64) -> i64 {
    if x == 0 { y } else { gcd(y % x, x) }
}

fn count_lattice_point_on(x0: i64, y0: i64, x1: i64, y1: i64) -> i64 {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    if dx == 0 {
        return max(0, dy - 1);
    }

    if dy == 0 {
        return max(0, dx - 1);
    }

    let g = gcd(dx, dy);

    g - 1
}

fn main() {
    type Mod = Mod1000000007;

    let n: usize = read();
    let xy = read_vec(n, || read_tuple!(i64, i64));

    let points = xy
        .citer()
        .sorted()
        .map(|(x, y)| Vector(x, y))
        .collect::<Vec<_>>();

    let pow2 = iterate(Mod::new(1), |&p| p * 2)
        .take(20000)
        .collect::<Vec<_>>();

    let pow2inv = pow2.citer().map(|x| x.inv()).collect::<Vec<_>>();

    let lower_nums = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    if j <= i {
                        0
                    } else {
                        (i + 1..j)
                            .filter(|&k| (points[k] - points[i]).cross(points[j] - points[k]) > 0)
                            .count()
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let lower = (0..n)
        .map(|i| {
            let mut d = vec![vec![(Mod::zero(), Mod::zero()); n]; n];

            for j in i + 1..n {
                let m =
                    count_lattice_point_on(points[i].0, points[i].1, points[j].0, points[j].1) + 1;
                if m % 2 == 0 {
                    d[i][j].0 = pow2inv[lower_nums[i][j] + 1];
                } else {
                    d[i][j].1 = pow2inv[lower_nums[i][j] + 1];
                }
            }

            for j in i..n {
                for k in j + 1..n {
                    for l in k + 1..n {
                        if (points[k] - points[j]).cross(points[l] - points[k]) <= 0 {
                            continue;
                        }

                        let m = count_lattice_point_on(
                            points[k].0,
                            points[k].1,
                            points[l].0,
                            points[l].1,
                        ) + 1;
                        if m % 2 == 0 {
                            d[k][l].0 = d[k][l].0 + d[j][k].0 * pow2inv[lower_nums[k][l] + 1];
                            d[k][l].1 = d[k][l].1 + d[j][k].1 * pow2inv[lower_nums[k][l] + 1];
                        } else {
                            d[k][l].0 = d[k][l].0 + d[j][k].1 * pow2inv[lower_nums[k][l] + 1];
                            d[k][l].1 = d[k][l].1 + d[j][k].0 * pow2inv[lower_nums[k][l] + 1];
                        }
                    }
                }
            }

            d
        })
        .collect::<Vec<_>>();

    let upper = (0..n)
        .map(|i| {
            let mut d = vec![vec![(Mod::zero(), Mod::zero()); n]; n];

            for j in 0..i {
                let m =
                    count_lattice_point_on(points[i].0, points[i].1, points[j].0, points[j].1) + 1;
                if m % 2 == 0 {
                    d[i][j].0 = pow2[lower_nums[j][i]];
                } else {
                    d[i][j].1 = pow2[lower_nums[j][i]];
                }
            }

            for j in (0..=i).rev() {
                for k in (0..j).rev() {
                    for l in (0..k).rev() {
                        if (points[k] - points[j]).cross(points[l] - points[k]) <= 0 {
                            continue;
                        }

                        let m = count_lattice_point_on(
                            points[k].0,
                            points[k].1,
                            points[l].0,
                            points[l].1,
                        ) + 1;
                        if m % 2 == 0 {
                            d[k][l].0 = d[k][l].0 + d[j][k].0 * pow2[lower_nums[l][k]];
                            d[k][l].1 = d[k][l].1 + d[j][k].1 * pow2[lower_nums[l][k]];
                        } else {
                            d[k][l].0 = d[k][l].0 + d[j][k].1 * pow2[lower_nums[l][k]];
                            d[k][l].1 = d[k][l].1 + d[j][k].0 * pow2[lower_nums[l][k]];
                        }
                    }
                }
            }

            d
        })
        .collect::<Vec<_>>();
    // eprintln!("{:?}", lower);
    // eprintln!("{:?}", upper);

    let ans = (0..n)
        .map(|i| {
            (i + 1..n)
                .map(|j| {
                    let (m0_e, m0_o) = (i..j)
                        .map(|k| lower[i][k][j])
                        .fold((Mod::zero(), Mod::zero()), |(m0_e, m0_o), (m_e, m_o)| {
                            (m0_e + m_e, m0_o + m_o)
                        });
                    let (m1_e, m1_o) = (i + 1..=j)
                        .map(|k| upper[j][k][i])
                        .fold((Mod::zero(), Mod::zero()), |(m1_e, m1_o), (m_e, m_o)| {
                            (m1_e + m_e, m1_o + m_o)
                        });

                    m0_e * 2 * m1_e + m0_o * 2 * m1_o - 1
                })
                .sum::<Mod>()
        })
        .sum::<Mod>();
    println!("{}", ans);
}
