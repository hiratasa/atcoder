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

// fn solve0(s: &[char], lr: &[(usize, usize)]) -> Mod1000000007 {
//     let n = s.len();
//     let _m = lr.len();

//     let mut v = once(s.to_vec()).collect::<FxHashSet<_>>();
//     for &(l, r) in lr {
//         let l = l - 1;

//         let v2 = v
//             .iter()
//             .flat_map(|s0| {
//                 s0[l..r].citer().permutations(r - l).map(move |w| {
//                     s0[..l]
//                         .citer()
//                         .chain(w)
//                         .chain(s0[r..].citer())
//                         .collect::<Vec<_>>()
//                 })
//             })
//             .collect::<FxHashSet<_>>();

//         v.extend(v2);
//     }

//     Mod::new(v.len())
// }

fn main() {
    type Mod = Mod1000000007;

    let (n, m) = read_tuple!(usize, usize);
    let s = read_str();

    let lr = read_vec(m, || read_tuple!(usize, usize));

    // use rand::Rng;
    // use rand::SeedableRng;
    // let mut rng = rand::rngs::SmallRng::from_entropy();
    // let (n, m) = (10, 3);
    // let s = "01".chars().cycle().take(n).collect::<Vec<_>>();
    // let lr = repeat_with(|| {
    //     let l = rng.gen_range(1, n - 1);
    //     let r = rng.gen_range(l + 1, n);
    //     (l, r)
    // })
    // .take(m)
    // .sorted_by_key(|&(l, _r)| l)
    // .collect::<Vec<_>>();

    let c = once(0)
        .chain(s.citer().map(|d| (d == '1') as usize))
        .cumsum::<usize>()
        .collect::<Vec<_>>();

    let combi = iterate(vec![Mod::one()], |prev| {
        once(Mod::one())
            .chain(izip!(prev.citer(), prev.citer().skip(1)).map(|(c0, c1)| c0 + c1))
            .chain(once(Mod::one()))
            .collect()
    })
    .take(n + 1)
    .collect::<Vec<_>>();

    let l0 = lr[0].0 - 1;
    let mut init = vec![Mod::zero(); n + 1];
    init[c[l0]] = Mod::one();

    let dp = lr
        .citer()
        .group_by(|(l, _r)| *l)
        .into_iter()
        .map(|(l, it)| (l, it.map(|(_l, r)| r).max().unwrap()))
        .scan(0, |prev_r, (l, r)| {
            if *prev_r < r {
                *prev_r = r;
                Some(Some((l, r)))
            } else {
                Some(None)
            }
        })
        .flatten()
        .map(|(l, r)| (l - 1, r))
        .chain(once((n, n)))
        .tuple_windows()
        .map(|((l, r), (l1, _r1))| (l, r, l1))
        .fold(init, |prev, (l, r, next_l)| {
            // eprintln!("{} {} {} {:?}", l, r, next_l, prev);
            let mut dp = vec![Mod::zero(); n + 1];
            for i in 0..=c[r] {
                let k = c[r] - i;

                // eprintln!(
                //     "{} {} {}-{}",
                //     i,
                //     k,
                //     k.saturating_sub(r.saturating_sub(next_l)),
                //     min(k, min(next_l, r) - l)
                // );
                for j in k.saturating_sub(r.saturating_sub(next_l))..=min(k, min(next_l, r) - l) {
                    // eprintln!("{} {} {} {}", i, j, prev[i], combi[min(next_l, r) - l][j]);
                    dp[i + j] = dp[i + j] + prev[i] * combi[min(next_l, r) - l][j];
                }
            }
            if next_l > r {
                let mut dp1 = vec![Mod::zero(); n + 1];
                dp1[c[next_l]] = dp[c[r]];
                dp1
            } else {
                dp
            }
        });
    // eprintln!("{:?}", dp);

    let ans = dp[c[n]];
    println!("{}", ans);

    // let ans0 = solve0(&s, &lr);
    // if ans != ans0 {
    //     eprintln!("ans={}, ans0={}", ans, ans0);
    //     eprintln!("{} {}", n, m);
    //     eprintln!("{}", s.citer().join(""));
    //     for (l, r) in lr {
    //         eprintln!("{} {}", l, r);
    //     }
    //     break;
    // }
}
