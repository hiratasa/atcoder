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
            .scan(Mod::new(1), |p, _| Some(std::mem::replace(p, *p * c)))
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
            .scan(Mod::new(1), |p, _| Some(std::mem::replace(p, *p * c)))
            .collect::<Vec<_>>(),
    );
    for x in f {
        *x /= n;
    }
}

// t *未満* の時間での移動方法の総数
fn solve<M: Modulus>(
    n: usize,
    t: usize,
    // 空のところは空vec
    p: &Vec<Vec<Vec<Mod<M>>>>,
) -> Vec<Vec<Vec<Mod<M>>>> {
    assert!(t.is_power_of_two());

    if t == 1 {
        return (0..n)
            .map(|i| (0..n).map(|j| vec![Mod::new((i == j) as usize)]).collect())
            .collect();
    }

    let t0 = 2 * t;

    let mut q = solve(n, t / 2, p);

    // eprintln!("{}: {:?}", t / 2, q);

    for i in 0..n {
        for j in 0..n {
            if j < i {
                q[i][j] = q[j][i].clone();
            } else {
                q[i][j].resize(t0, Mod::zero());
                fft_mod(&mut q[i][j]);
            }
        }
    }

    let mut pt = vec![vec![vec![]; n]; n];
    for i in 0..n {
        for j in 0..n {
            if j < i {
                pt[i][j] = pt[j][i].clone();
            } else if !p[i][j].is_empty() {
                // pはt要素取って残りはゼロ埋め
                pt[i][j] = p[i][j]
                    .citer()
                    .take(t)
                    .chain(repeat(Mod::zero()))
                    .take(t0)
                    .collect();
                fft_mod(&mut pt[i][j]);
            }
        }
    }

    let mul = |lhs: &Vec<Vec<Vec<Mod<M>>>>, rhs: &Vec<Vec<Vec<Mod<M>>>>| {
        (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| {
                        (0..n).fold(vec![Mod::zero(); t0], |mut r, k| {
                            // 空のところは計算省略
                            if !lhs[i][k].is_empty() && !rhs[k][j].is_empty() {
                                izip!(r.iter_mut(), lhs[i][k].citer(), rhs[k][j].citer())
                                    .for_each(|(x, y, z)| *x += y * z);
                            }
                            r
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    };

    // q^2
    let q2 = mul(&q, &q);

    // qpq
    let qpq = mul(&q, &mul(&pt, &q));

    // 2q - q^2 + qpq
    let mut r = q.clone();
    for i in 0..n {
        for j in 0..n {
            if j < i {
                r[i][j] = r[j][i].clone();
            } else {
                for tt in 0..t0 {
                    r[i][j][tt] *= 2;
                    r[i][j][tt] += -q2[i][j][tt] + qpq[i][j][tt];
                }

                inv_fft_mod(&mut r[i][j]);
                r[i][j].resize(t, Mod::zero());
            }
        }
    }

    r
}

// fn solve0<M: Modulus>(n: usize, t: usize, p: &Vec<Vec<Vec<Mod<M>>>>) -> Vec<Vec<Vec<Mod<M>>>> {
//     let mul = |lhs: &Vec<Vec<Vec<Mod<M>>>>, rhs: &Vec<Vec<Vec<Mod<M>>>>| {
//         (0..n)
//             .map(|i| {
//                 (0..n)
//                     .map(|j| {
//                         (0..n).fold(vec![Mod::zero(); t], |mut r, k| {
//                             // 空のところは計算省略
//                             if !lhs[i][k].is_empty() && !rhs[k][j].is_empty() {
//                                 izip!(
//                                     r.iter_mut(),
//                                     (0..t).map(|idx| {
//                                         (0..=idx)
//                                             .map(|idx2| {
//                                                 lhs[i][k].get(idx2).copied().unwrap_or(Mod::zero())
//                                                     * rhs[k][j]
//                                                         .get(idx - idx2)
//                                                         .copied()
//                                                         .unwrap_or(Mod::zero())
//                                             })
//                                             .sum::<Mod<M>>()
//                                     })
//                                 )
//                                 .for_each(|(x, y)| *x += y);
//                             }
//                             r
//                         })
//                     })
//                     .collect::<Vec<_>>()
//             })
//             .collect::<Vec<_>>()
//     };

//     let mut r = vec![vec![vec![Mod::zero(); t]; n]; n];
//     let mut s = (0..n)
//         .map(|i| {
//             (0..n)
//                 .map(|j| vvec![Mod::new((i == j) as usize); Mod::zero(); t])
//                 .collect::<Vec<_>>()
//         })
//         .collect::<Vec<_>>();
//     for _ in 0..t {
//         for i in 0..n {
//             for j in 0..n {
//                 for tt in 0..t {
//                     r[i][j][tt] += s[i][j][tt];
//                 }
//             }
//         }

//         s = mul(&s, &p);
//     }

//     r
// }

fn main() {
    type Mod = Mod998244353;

    let (n, m, t) = read_tuple!(usize, usize, usize);

    // use rand::distr::Distribution;
    // use rand::SeedableRng;
    // let mut rng = rand::rngs::SmallRng::seed_from_u64(43);

    let p = (0..m).fold(vec![vec![vec![]; n]; n], |mut p, _| {
        let (a, b) = read_tuple!(usize, usize);
        p[a - 1][b - 1] = read_row::<Mod>();
        // let dist = rand::distr::Uniform::new(1, n).unwrap();
        // let a = dist.sample(&mut rng);
        // let dist1 = rand::distr::Uniform::new(a + 1, n + 1).unwrap();
        // let b = dist1.sample(&mut rng);
        // let dist2 = rand::distr::Uniform::new(0, 100).unwrap();
        // p[a - 1][b - 1] = (0..t)
        //     .map(|_| Mod::new(dist2.sample(&mut rng)))
        //     .collect::<Vec<_>>();

        p[a - 1][b - 1].insert(0, Mod::zero());
        p[b - 1][a - 1] = p[a - 1][b - 1].clone();

        p
    });

    let r = solve(n, (t + 1).next_power_of_two(), &p);
    println!("{}", r[0][0][t]);

    // let r = solve0(n, t + 1, &p);
    // println!("{}", r[0][0][t]);
}
