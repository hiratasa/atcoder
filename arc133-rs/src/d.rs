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
use itertools::{chain, iproduct, iterate, izip, Itertools};
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

// fn solve0(l: usize, r: usize, v: usize, b0: usize, b1: usize) -> Mod998244353 {
//     Mod::new(
//         (l..=r)
//             .flat_map(|ll| (ll..=r).map(move |rr| (ll, rr)))
//             .filter(|&(ll, rr)| ll & 1 == b0 && rr & 1 == b1)
//             .filter(|&(ll, rr)| (ll..=rr).fold(0, |x, y| x ^ y) == v)
//             .inspect(|&(ll, rr)| eprintln!("{} {}", ll, rr))
//             .count(),
//     )
// }

// l<=ll<rr<=r で ll^rr = v となる個数
fn solve1(
    l: usize,
    r: usize,
    v: usize,
    memo: &mut FxHashMap<(usize, usize, usize), Mod998244353>,
) -> Mod998244353 {
    if l >= r {
        return Mod::zero();
    }

    if r < 2 {
        return Mod::new(
            (l..=r)
                .flat_map(|ll| (ll + 1..=r).map(move |rr| (ll, rr)))
                .filter(|&(ll, rr)| ll ^ rr == v)
                .count(),
        );
    }

    if let Some(&ret) = memo.get(&(l, r, v)) {
        return ret;
    }

    let ret = iproduct!(0..2, 0..2)
        .filter(|&(i, j)| i ^ j == v & 1)
        .map(|(i, j)| {
            let l1 = (l + 2 - 1 - i) / 2 * 2 + i;
            let r1 = (r - j) / 2 * 2 + j;
            if l1 < r1 {
                if v >> 1 == 0 && i < j {
                    solve1(l1 >> 1, r1 >> 1, v >> 1, memo) + ((r1 >> 1) - (l1 >> 1) + 1)
                } else {
                    solve1(l1 >> 1, r1 >> 1, v >> 1, memo)
                }
            } else {
                Mod::zero()
            }
        })
        .sum();

    memo.insert((l, r, v), ret);

    ret
}

fn main() {
    type Mod = Mod998244353;

    let (l, r, v) = read_tuple!(usize, usize, usize);

    let ans00 = iproduct!(0..4, 0..4)
        .filter(|&(i, j)| i & 1 == 0 && j & 1 == 0)
        .filter(|&(i, j)| (i..=4 + j).fold(0, |x, y| x ^ y) % 2 == v & 1)
        .map(|(i, j)| {
            it![v / 2 * 2, v / 2 * 2 + 1]
                .filter(|&x| x % 4 == j)
                .filter(|&x| l <= x && x <= r)
                .map(|x| (x + 4 - i) / 4 - (l - 1 + 4 - i) / 4)
                .map_into::<Mod>()
                .sum::<Mod>()
        })
        .sum::<Mod>();

    let ans01 = if v < 2 {
        iproduct!(0..4, 0..4)
            .filter(|&(i, j)| i & 1 == 0 && j & 1 == 1)
            .filter(|&(i, j)| (i..=4 + j).fold(0, |x, y| x ^ y) % 2 == v & 1)
            .map(|(i, j)| {
                let x0 = (l / 4 * 4 + 4 + i - l) % 4 + l;
                let m1 = (r + 4 - j) / 4 - (x0 - 1 + 4 - j) / 4;

                Mod::new(m1) * Mod::new(m1 + 1) / 2
            })
            .sum::<Mod>()
    } else {
        Mod::zero()
    };

    let mut memo = FxHashMap::default();
    let ans10 = iproduct!(0..4, 0..4)
        .filter(|&(i, j)| i & 1 == 1 && j & 1 == 0)
        .filter(|&(i, j)| (i..=4 + j).fold(0, |x, y| x ^ y) % 2 == v & 1)
        .filter(|&(i, j)| (i >> 1) ^ (j >> 1) == (v >> 1) & 1)
        .filter(|&(_i, j)| j <= r)
        .map(|(i, j)| {
            let l1 = (l + 4 - 1 - i) / 4 * 4 + i;
            let r1 = (r - j) / 4 * 4 + j;
            if l1 <= r1 {
                if v >> 2 == 0 && i <= j {
                    solve1(l1 >> 2, r1 >> 2, v >> 2, &mut memo) + ((r1 >> 2) - (l1 >> 2) + 1)
                } else {
                    solve1(l1 >> 2, r1 >> 2, v >> 2, &mut memo)
                }
            } else {
                Mod::zero()
            }
        })
        .sum::<Mod>();

    let ans11 = iproduct!(0..4, 0..4)
        .filter(|&(i, j)| i & 1 == 1 && j & 1 == 1)
        .filter(|&(i, j)| (i..=4 + j).fold(0, |x, y| x ^ y) % 2 == v & 1)
        .map(|(i, j)| {
            it![v / 2 * 2, v / 2 * 2 + 1]
                .filter(|&x| x % 4 == i)
                .filter(|&x| l <= x && x <= r)
                .map(|x| (r + 4 - j) / 4 - (x - 1 + 4 - j) / 4)
                .map_into::<Mod>()
                .sum::<Mod>()
        })
        .sum::<Mod>();

    let ans = ans00 + ans01 + ans10 + ans11;
    println!("{}", ans);

    // let ans0 = iproduct!(0..2, 0..2)
    //     .map(|(i, j)| {
    //         eprintln!("==({}, {})==", i, j);
    //         solve0(l, r, v, i, j)
    //     })
    //     .sum::<Mod>();
}