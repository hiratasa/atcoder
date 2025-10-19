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

fn mul_mat<M: Modulus>(x: &Vec<Vec<Mod<M>>>, y: &Vec<Vec<Mod<M>>>) -> Vec<Vec<Mod<M>>> {
    let n = x.len();

    let mut r = vec![vec![Mod::zero(); n]; n];
    for i in 0..n {
        for k in 0..n {
            for j in 0..n {
                r[i][j] += x[i][k] * y[k][j];
            }
        }
    }

    r
}

fn pow_mat<M: Modulus>(mut x: Vec<Vec<Mod<M>>>, mut p: usize) -> Vec<Vec<Mod<M>>> {
    let n = x.len();

    let mut y = vec![vec![Mod::zero(); n]; n];
    for i in 0..n {
        y[i][i] = Mod::one();
    }

    while p > 0 {
        if p & 1 > 0 {
            y = mul_mat(&x, &y);
        }

        x = mul_mat(&x, &x);
        p >>= 1;
    }

    y
}

fn solve0(s: &[char], k: usize) -> Mod1000000007 {
    type Mod = Mod1000000007;

    let n = s.len();

    let mut dp0 = vec![vec![Mod::zero(); 1]; 2];
    match s[0] {
        '0' => {
            dp0[0][0] = Mod::one();
        }
        '1' => {
            dp0[1][0] = Mod::one();
        }
        '?' => {
            dp0[0][0] = Mod::one();
            dp0[1][0] = Mod::one();
        }
        _ => unreachable!(),
    }

    let m = n * k;
    let dp = s
        .citer()
        .cycle()
        .take(m)
        .skip(1)
        .fold(dp0, |dp, c| match c {
            '0' => vec![
                izip!(
                    dp[0].citer().chain(once(Mod::zero())),
                    once(Mod::zero()).chain(dp[1].citer()),
                )
                .map(|(x, y)| x + y)
                .take(m)
                .collect::<Vec<_>>(),
                vec![Mod::zero(); m],
            ],
            '1' => vec![
                vec![Mod::zero(); m],
                izip!(
                    once(Mod::zero()).chain(dp[0].citer()),
                    dp[1].citer().chain(once(Mod::zero())),
                )
                .map(|(x, y)| x + y)
                .take(m)
                .collect::<Vec<_>>(),
            ],
            '?' => vec![
                izip!(
                    dp[0].citer().chain(once(Mod::zero())),
                    once(Mod::zero()).chain(dp[1].citer()),
                )
                .map(|(x, y)| x + y)
                .take(m)
                .collect::<Vec<_>>(),
                izip!(
                    once(Mod::zero()).chain(dp[0].citer()),
                    dp[1].citer().chain(once(Mod::zero())),
                )
                .map(|(x, y)| x + y)
                .take(m)
                .collect::<Vec<_>>(),
            ],
            _ => unreachable!(),
        });

    dp.iter()
        .map(|row| {
            row.citer()
                .enumerate()
                .map(|(i, x)| x * ((i + 1) / 2))
                .sum::<Mod>()
        })
        .sum()
}

fn main() {
    type Mod = Mod1000000007;

    let s = read_str();
    let k: usize = read();

    // 前半: パターン数、後半: パターン数×操作回数
    // 0: 最後が0, 境目が偶数個
    // 1: 最後が0, 境目が奇数個
    // 2: 最後が1, 境目が偶数個
    // 3: 最後が1, 境目が奇数個
    let mut m0 = vec![vec![Mod::zero(); 8]; 8];
    for i in 0..8 {
        m0[i][i] = Mod::one();
    }

    let m = s.citer().fold(m0, |m, c| match c {
        '0' => (0..8)
            .map(|i| {
                vec![
                    m[i][0] + m[i][3],
                    m[i][1] + m[i][2],
                    Mod::zero(),
                    Mod::zero(),
                    m[i][4] + m[i][7] + m[i][3],
                    m[i][5] + m[i][6],
                    Mod::zero(),
                    Mod::zero(),
                ]
            })
            .collect::<Vec<_>>(),
        '1' => (0..8)
            .map(|i| {
                vec![
                    Mod::zero(),
                    Mod::zero(),
                    m[i][1] + m[i][2],
                    m[i][0] + m[i][3],
                    Mod::zero(),
                    Mod::zero(),
                    m[i][5] + m[i][1] + m[i][6],
                    m[i][4] + m[i][7],
                ]
            })
            .collect::<Vec<_>>(),
        '?' => (0..8)
            .map(|i| {
                vec![
                    m[i][0] + m[i][3],
                    m[i][1] + m[i][2],
                    m[i][1] + m[i][2],
                    m[i][0] + m[i][3],
                    m[i][4] + m[i][7] + m[i][3],
                    m[i][5] + m[i][6],
                    m[i][5] + m[i][1] + m[i][6],
                    m[i][4] + m[i][7],
                ]
            })
            .collect::<Vec<_>>(),
        _ => unreachable!(),
    });

    eprintln!("{:?}", m);
    let mm = pow_mat(m, k);
    eprintln!("{:?}", mm);

    let ans = match s[0] {
        '0' => mm[0][4] + (mm[0][1] + mm[0][5]) + mm[0][6] + (mm[0][3] + mm[0][7]),
        '1' => mm[2][4] + (mm[2][1] + mm[2][5]) + mm[2][6] + (mm[2][3] + mm[2][7]),
        '?' => mm[0][4] + mm[0][6] + mm[2][4] + mm[2][6],
        _ => unreachable!(),
    };
    println!("{}", ans);

    // let ans0 = solve0(&s, k);
    // eprintln!("{}", ans0);
}
