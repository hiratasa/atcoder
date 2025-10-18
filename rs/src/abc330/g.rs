use std::iter::once;

use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;
use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    type Mod = Mod998244353;

    input! {
        n: usize,
        a: [i64; n],
    };

    if n == 1 {
        println!("0");
        return;
    }

    let (fact, inv, inv_fact) = generate_fact(n);

    let a = a
        .into_iter()
        .map(|x| x.try_into().ok())
        .map(|x: Option<usize>| x.map(|x| x - 1))
        .collect::<Vec<_>>();

    let inversion = Mod::new(
        a.iter()
            .flatten()
            .tuple_combinations()
            .filter(|&(x, y)| x > y)
            .count(),
    );

    let k = n - a.iter().flatten().count();

    if k == 0 {
        println!("{}", inversion * inversion);
        return;
    }

    let num_blanks = once(0)
        .chain(a.iter().map(|x| x.is_none() as usize))
        .cumsum::<usize>()
        .collect::<Vec<_>>();
    let mut num_belows = a
        .iter()
        .copied()
        .flatten()
        .fold(vec![1; n + 1], |mut t, x| {
            t[x + 1] = 0;
            t
        });
    num_belows[0] = 0;
    for i in 1..=n {
        num_belows[i] += num_belows[i - 1];
    }

    let b = a
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(i, x_opt)| x_opt.map(|x| (i, x)))
        .collect::<Vec<_>>();

    let calc2 = |x0: bool, x1: bool| {
        if x0 && x1 {
            inversion * fact[k]
        } else if x0 {
            b.iter()
                .copied()
                .map(|(i, x)| (num_blanks[n] - num_blanks[i]) * num_belows[x] * fact[k - 1])
                .sum::<Mod>()
        } else if x1 {
            b.iter()
                .copied()
                .map(|(i, x)| num_blanks[i] * (num_belows[n] - num_belows[x]) * fact[k - 1])
                .sum::<Mod>()
        } else {
            k * (k - 1) / 2 * fact[k] * inv[2]
        }
    };

    let pos_ords = (0..4)
        .map(|_| (0..4))
        .multi_cartesian_product()
        .filter(|ord| ord[0] < ord[1] && ord[2] < ord[3])
        .filter(|ord| {
            let mi = *ord.iter().min().unwrap();
            let ma = *ord.iter().max().unwrap();

            mi == 0 && (mi..=ma).all(|i| ord.contains(&i))
        })
        .collect::<Vec<_>>();
    let val_ords = pos_ords
        .iter()
        .cloned()
        .map(|mut ord| {
            ord.swap(0, 1);
            ord.swap(2, 3);
            ord
        })
        .collect::<Vec<_>>();

    let asc_pairs = (0..n)
        .map(|i| {
            if a[i].is_some() {
                (i + 1..n).filter(|&j| a[i] < a[j]).collect::<Vec<_>>()
            } else {
                vec![]
            }
        })
        .collect::<Vec<_>>();
    let desc_pairs = (0..n)
        .map(|i| {
            if a[i].is_some() {
                (i + 1..n)
                    .filter(|&j| a[j].is_some() && a[i] > a[j])
                    .collect::<Vec<_>>()
            } else {
                vec![]
            }
        })
        .collect::<Vec<_>>();

    let mut patterns = FxHashMap::default();
    let mut ans = iproduct!([false, true], [false, true], [false, true], [false, true])
        .map(|(x0, x1, x2, x3)| {
            if x0 && x1 {
                calc2(x2, x3) * inversion
            } else if x2 && x3 {
                calc2(x0, x1) * inversion
            } else if (x0 || x1 || x2 || x3) && k == n {
                Mod::zero()
            } else {
                let fixed = [x0, x1, x2, x3];
                iproduct!(&pos_ords, &val_ords)
                    .filter(|(pos_ord, val_ord)| {
                        (0..4)
                            .tuple_combinations()
                            .all(|(i, j)| (pos_ord[i] == pos_ord[j]) == (val_ord[i] == val_ord[j]))
                    })
                    .filter(|(pos_ord, _val_ord)| {
                        (0..4)
                            .tuple_combinations()
                            .all(|(i, j)| fixed[i] == fixed[j] || pos_ord[i] != pos_ord[j])
                    })
                    .filter(|(pos_ord, _)| {
                        (0..4)
                            .filter(|&i| !fixed[i] && (0..i).all(|j| pos_ord[j] != pos_ord[i]))
                            .count()
                            <= k
                    })
                    .for_each(|(pos_ord, val_ord)| {
                        let first_idxs = (0..4)
                            .filter(|&i| !fixed[i] && (0..i).all(|j| pos_ord[j] != pos_ord[i]))
                            .collect::<Vec<_>>();

                        let pos_ranges = first_idxs
                            .iter()
                            .map(|&i| {
                                let b = (0..4)
                                    .filter(|&j| pos_ord[j] < pos_ord[i])
                                    .filter(|&j| fixed[j])
                                    .max_by_key(|&j| pos_ord[j]);
                                let e = (0..4)
                                    .filter(|&j| pos_ord[j] > pos_ord[i])
                                    .filter(|&j| fixed[j])
                                    .min_by_key(|&j| pos_ord[j]);

                                (b, e)
                            })
                            .collect::<Vec<_>>();
                        let val_ranges = first_idxs
                            .iter()
                            .map(|&i| {
                                let l = (0..4)
                                    .filter(|&j| val_ord[j] < val_ord[i])
                                    .filter(|&j| fixed[j])
                                    .max_by_key(|&j| val_ord[j]);
                                let u = (0..4)
                                    .filter(|&j| val_ord[j] > val_ord[i])
                                    .filter(|&j| fixed[j])
                                    .min_by_key(|&j| val_ord[j]);

                                (l, u)
                            })
                            .collect::<Vec<_>>();

                        let p_coeffs = pos_ranges
                            .iter()
                            .copied()
                            .sorted()
                            .group_by(|&r| r)
                            .into_iter()
                            .map(|((b, e), it)| (b, e, it.count()))
                            .collect::<Vec<_>>();
                        let v_coeffs = val_ranges
                            .iter()
                            .copied()
                            .sorted()
                            .group_by(|&r| r)
                            .into_iter()
                            .map(|((b, e), it)| (b, e, it.count()))
                            .collect::<Vec<_>>();

                        if let Some(idx0) = (0..4).find(|&idx| fixed[idx]) {
                            if let Some(idx1) = (idx0 + 1..4)
                                .find(|&idx| fixed[idx] && val_ord[idx0] != val_ord[idx])
                            {
                                let (idx0, idx1) = if pos_ord[idx0] <= pos_ord[idx1] {
                                    (idx0, idx1)
                                } else {
                                    (idx1, idx0)
                                };

                                let pp = p_coeffs
                                    .iter()
                                    .copied()
                                    .map(|(b, e, z)| {
                                        let b = b.map(|b| {
                                            [idx0, idx1].iter().position(|&idx| idx == b).unwrap()
                                        });
                                        let e = e.map(|b| {
                                            [idx0, idx1].iter().position(|&idx| idx == b).unwrap()
                                        });

                                        (b, e, z)
                                    })
                                    .sorted()
                                    .collect::<Vec<_>>();
                                let vv = v_coeffs
                                    .iter()
                                    .copied()
                                    .map(|(b, e, z)| {
                                        let b = b.map(|b| {
                                            [idx0, idx1].iter().position(|&idx| idx == b).unwrap()
                                        });
                                        let e = e.map(|b| {
                                            [idx0, idx1].iter().position(|&idx| idx == b).unwrap()
                                        });

                                        (b, e, z)
                                    })
                                    .sorted()
                                    .collect::<Vec<_>>();

                                *patterns
                                    .entry((
                                        first_idxs.len(),
                                        2,
                                        val_ord[idx0] < val_ord[idx1],
                                        pp,
                                        vv,
                                    ))
                                    .or_insert(0) += 1;
                            } else {
                                let pp = p_coeffs
                                    .iter()
                                    .copied()
                                    .map(|(b, e, z)| {
                                        let b = b.map(|_| 0);
                                        let e = e.map(|_| 0);

                                        (b, e, z)
                                    })
                                    .collect::<Vec<_>>();
                                let vv = v_coeffs
                                    .iter()
                                    .copied()
                                    .map(|(b, e, z)| {
                                        let b = b.map(|_| 0);
                                        let e = e.map(|_| 0);

                                        (b, e, z)
                                    })
                                    .collect::<Vec<_>>();

                                *patterns
                                    .entry((first_idxs.len(), 1, false, pp, vv))
                                    .or_insert(0) += 1;
                            }
                        } else {
                            *patterns
                                .entry((first_idxs.len(), 0, false, p_coeffs, v_coeffs))
                                .or_insert(0) += 1;
                        }
                    });

                Mod::zero()
            }
        })
        .sum::<Mod>();

    let calc = |nums: &[(usize, usize)],
                pp: &[(Option<usize>, Option<usize>, usize)],
                vv: &[(Option<usize>, Option<usize>, usize)],
                l: usize| {
        let pr = pp
            .iter()
            .map(|&(b, e, c)| {
                let b = b.map_or(0, |i| nums[i].0 + 1);
                let e = e.map_or(n, |i| nums[i].0);

                let d = num_blanks[e] - num_blanks[b];

                if c > d {
                    Mod::zero()
                } else if c == 1 {
                    Mod::raw(d)
                } else if c == 2 {
                    Mod::raw(d * (d - 1) / 2)
                } else {
                    fact[d] * inv_fact[c] * inv_fact[d - c]
                }
            })
            .product::<Mod998244353>();

        let vr = vv
            .iter()
            .map(|&(b, e, c)| {
                let b = b.map_or(0, |i| nums[i].1 + 1);
                let e = e.map_or(n, |i| nums[i].1);

                let d = num_belows[e] - num_belows[b];

                if c > d {
                    Mod::zero()
                } else if c == 1 {
                    Mod::raw(d)
                } else if c == 2 {
                    Mod::raw(d * (d - 1) / 2)
                } else {
                    fact[d] * inv_fact[c] * inv_fact[d - c]
                }
            })
            .product::<Mod998244353>();

        pr * vr * fact[k - l]
    };

    for ((l, m, less, pp, vv), c) in patterns {
        let mut nums = vec![(0, 0); m];

        let r = match m {
            0 => calc(&nums, &pp, &vv, l) * c,
            1 => b
                .iter()
                .copied()
                .map(|(i, x)| {
                    nums[0] = (i, x);
                    calc(&nums, &pp, &vv, l) * c
                })
                .sum::<Mod>(),
            2 => {
                b.iter()
                    .copied()
                    .map(|(i, x)| {
                        nums[0] = (i, x);
                        if less {
                            asc_pairs[i]
                                .iter()
                                .copied()
                                .map(|j| {
                                    let y = a[j].unwrap();
                                    nums[1] = (j, y);

                                    calc(&nums, &pp, &vv, l)
                                })
                                .sum::<Mod>()
                        } else {
                            desc_pairs[i]
                                .iter()
                                .copied()
                                .map(|j| {
                                    let y = a[j].unwrap();
                                    nums[1] = (j, y);

                                    calc(&nums, &pp, &vv, l)
                                })
                                .sum::<Mod>()
                        }
                    })
                    .sum::<Mod>()
                    * c
            }
            _ => unreachable!(),
        };

        // eprintln!("{l} {m} {less} {pp:?} {vv:?} {c}; {r}");

        ans += r;
    }

    println!("{ans}");
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
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
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
    pub fn raw(n: usize) -> Self {
        Mod(n, std::marker::PhantomData)
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
        if self.0 == 0 {
            Mod::raw(0)
        } else {
            Mod::raw(M::modulus() - self.0)
        }
    }
}
impl<M: Modulus> std::ops::Add<Mod<M>> for Mod<M> {
    type Output = Self;
    fn add(self, rhs: Mod<M>) -> Self {
        let r = self.0 + rhs.0;
        if r < M::modulus() {
            Mod::raw(r)
        } else {
            Mod::raw(r - M::modulus())
        }
    }
}
impl<M: Modulus> std::ops::Add<usize> for Mod<M> {
    type Output = Self;
    fn add(self, rhs: usize) -> Self {
        self + Mod::new(rhs)
    }
}
impl<M: Modulus> std::ops::Add<Mod<M>> for usize {
    type Output = Mod<M>;
    fn add(self, rhs: Mod<M>) -> Mod<M> {
        Mod::new(self) + rhs.0
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
        let r = self.0.wrapping_sub(rhs.0);
        if r < M::modulus() {
            Mod::raw(r)
        } else {
            Mod::raw(r.wrapping_add(M::modulus()))
        }
    }
}
impl<M: Modulus> std::ops::Sub<usize> for Mod<M> {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self {
        self - Mod::new(rhs)
    }
}
impl<M: Modulus> std::ops::Sub<Mod<M>> for usize {
    type Output = Mod<M>;
    fn sub(self, rhs: Mod<M>) -> Mod<M> {
        Mod::new(self) - rhs
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
impl<M: Modulus> rand::distr::Distribution<Mod<M>> for rand::distr::StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Mod<M> {
        Mod::new(rng.random_range(0..M::modulus()))
    }
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
