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

#[allow(dead_code)]
fn is_good(s: &[char]) -> bool {
    let factors = (1..)
        .take_while(|&x| x * x <= s.len())
        .filter(|&x| s.len() % x == 0)
        .flat_map(|x| it!(x, s.len() / x))
        .dedup()
        .collect::<Vec<_>>();
    let z = z_algorithm(&s);

    factors.citer().filter(|&l| s.len() / l >= 2).all(|l| {
        (0..)
            .map(|i| i * l)
            .take_while(|&i| i < s.len())
            .any(|i| z[i] < l)
    })
}

fn z_algorithm<T: std::cmp::Eq>(s: &[T]) -> Vec<usize> {
    let n = s.len();

    // z[i] = max_{j<n} s[0:j] = s[i:i+j]
    let mut z = vec![0; n];
    z[0] = n;

    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
        // assert!(s[l..r] == s[0..r - l]);
        if i < r && z[i - l] < r - i {
            z[i] = z[i - l];
        } else {
            // i < rなら、 z[i - l] >= r - i なので、
            // s[i..r] (=s[i-l..r-l]) = s[0..r-i] が保証されている
            // i >= r なら再計算
            l = i;
            r = std::cmp::max(i, r);
            while r < n && s[r] == s[r - l] {
                r += 1;
            }
            z[i] = r - l;
        }
    }

    z
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

fn solve(w: &[char]) -> (usize, Mod1000000007) {
    if w.citer().all_equal() {
        return (w.len(), Mod::one());
    }

    let factors = (1..)
        .take_while(|&x| x * x <= w.len())
        .filter(|&x| w.len() % x == 0)
        .flat_map(|x| it!(x, w.len() / x))
        .dedup()
        .sorted()
        .collect::<Vec<_>>();
    let z = z_algorithm(&w);

    if factors.citer().filter(|&x| w.len() / x >= 2).all(|f| {
        (0..)
            .map(|i| i * f)
            .take_while(|&i| i < w.len())
            .any(|i| z[i] < f)
    }) {
        return (1, Mod::one());
    }

    let wr = w.citer().rev().collect::<Vec<_>>();

    let z = z_algorithm(&w);
    let zr = z_algorithm(&wr);

    let good = (1..)
        .take_while(|&j| 2 * j <= w.len())
        .fold(vec![true; w.len()], |mut good, j| {
            (1..)
                .take_while(|&k| j * k < w.len())
                .take_while(|&k| z[j * k] >= j)
                .for_each(|k| {
                    good[j * (k + 1) - 1] = false;
                });
            good
        });

    let goodr = (1..)
        .take_while(|&j| 2 * j <= w.len())
        .fold(vec![true; w.len()], |mut good, j| {
            (1..)
                .take_while(|&k| j * k < w.len())
                .take_while(|&k| zr[j * k] >= j)
                .for_each(|k| {
                    good[j * (k + 1) - 1] = false;
                });
            good
        });

    let x = (0..w.len() - 1)
        .filter(|&i| good[i] && goodr[w.len() - 2 - i])
        .count();

    (2, Mod::new(x))
}

fn solve2(w: &[char]) -> (usize, Mod1000000007) {
    let dp = (0..w.len()).fold(
        vvec![(0, Mod::one()); (usize::MAX, Mod::zero()); w.len() + 1],
        |mut dp, i| {
            let z = z_algorithm(&w[i..]);

            let good = (1..).take_while(|&j| i + 2 * j <= w.len()).fold(
                vec![true; w.len() - i],
                |mut good, j| {
                    (1..)
                        .take_while(|&k| i + j * k < w.len())
                        .take_while(|&k| z[j * k] >= j)
                        .for_each(|k| {
                            good[j * (k + 1) - 1] = false;
                        });
                    good
                },
            );

            (i..w.len()).filter(|&j| good[j - i]).for_each(|j| {
                match dp[j + 1].0.cmp(&(dp[i].0 + 1)) {
                    Ordering::Less => {
                        // NOP
                    }
                    Ordering::Equal => {
                        dp[j + 1].1 = dp[j + 1].1 + dp[i].1;
                    }
                    Ordering::Greater => {
                        dp[j + 1].0 = dp[i].0 + 1;
                        dp[j + 1].1 = dp[i].1;
                    }
                }
            });

            dp
        },
    );

    dp[w.len()]
}

#[allow(dead_code)]
fn check() {
    use rand::{Rng, SeedableRng};
    let mut rng = rand::rngs::SmallRng::from_os_rng();
    for _ in 0..10000000 {
        let n = rng.random_range(1..5);
        let t = repeat_with(|| rng.random_range(b'a'..=b'z') as char)
            .take(n)
            .collect::<Vec<_>>();
        let m = rng.random_range(2..5);
        let w = repeat(t.citer())
            .take(m)
            .flatten()
            .take(10000)
            .collect::<Vec<_>>();

        let ans = solve(&w);
        let ans2 = solve2(&w);

        if ans != ans2 {
            println!("{} x {}", t.citer().join(""), m);
            println!("{:?}", ans);
            println!("{:?}", ans2);
            return;
        }
    }
}

fn main() {
    let w = read_str();

    let ans = solve(&w);
    println!("{}", ans.0);
    println!("{}", ans.1);
}
