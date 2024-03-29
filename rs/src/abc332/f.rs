use itertools::Itertools;
use num::{One, Zero};
use proconio::{input, marker::Usize1};
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
impl<M: Modulus> rand::distributions::Distribution<Mod<M>> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Mod<M> {
        Mod::new(rng.gen_range(0..M::modulus()))
    }
}

fn range(r: impl std::ops::RangeBounds<usize>, n: usize) -> (usize, usize) {
    let start = match r.start_bound() {
        std::ops::Bound::Excluded(&i) => i + 1,
        std::ops::Bound::Included(&i) => i,
        std::ops::Bound::Unbounded => 0,
    };
    let end = match r.end_bound() {
        std::ops::Bound::Excluded(&i) => i,
        std::ops::Bound::Included(&i) => i + 1,
        std::ops::Bound::Unbounded => n,
    };
    (start, end)
}
fn segtree_table<M: Monoid + std::fmt::Debug, Op: Monoid + std::fmt::Debug>(
    values: &[M],
    lazy: &[Op],
) -> String {
    let cap = (values.len() + 1) / 2;
    assert!(cap.is_power_of_two());
    let height = cap.trailing_zeros() as usize + 1;
    let enable_lazy = !lazy.is_empty();
    let rows = (0..height)
        .map(|i| {
            ((1usize << i) - 1..(1 << (i + 1)) - 1)
                .map(|idx| {
                    if enable_lazy {
                        format!("{:?}; {:?}", values[idx], lazy[idx])
                    } else {
                        format!("{:?}", values[idx])
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let width = rows
        .iter()
        .enumerate()
        .map(|(i, cols)| {
            let k = cap >> i;
            cols.iter()
                .map(|s| s.len())
                .map(|l| (l.saturating_sub(k - 1) + k - 1) / k)
                .max()
                .unwrap()
        })
        .chain(once(2))
        .max()
        .unwrap();
    use std::iter::once;
    once(
        (0..cap)
            .map(|i| format!("|{:>width$}", i))
            .chain(once("|\n".to_string()))
            .collect::<String>(),
    )
    .chain(once(
        once("|")
            .chain((0..cap * (width + 1) - 1).map(|_| "+"))
            .chain(once("|\n"))
            .collect::<String>(),
    ))
    .chain(rows.iter().enumerate().map(|(i, cols)| {
        let k = cap >> i;
        let w = width * k + (k - 1);
        cols.iter()
            .map(|t| format!("|{:^w$}", t))
            .collect::<String>()
            + "|\n"
    }))
    .collect()
}
trait Monoid {
    fn id() -> Self;
    fn op(&self, rhs: &Self) -> Self;
}
#[derive(Debug)]
struct DualSegmentTree<Op>
where
    Op: Monoid,
{
    #[allow(dead_code)]
    height: usize,
    cap: usize,
    n: usize,
    lazy: Vec<Op>,
}
#[allow(dead_code)]
impl<Op> DualSegmentTree<Op>
where
    Op: Monoid + Clone,
{
    fn new(n: usize) -> Self {
        let cap = n.next_power_of_two();
        DualSegmentTree {
            height: cap.trailing_zeros() as usize + 1,
            n,
            cap,
            lazy: vec![Op::id(); 2 * cap - 1],
        }
    }
    fn with<T>(vals: &[T]) -> Self
    where
        T: Into<Op> + Clone,
    {
        let n = vals.len();
        let cap = n.next_power_of_two();
        let mut lazy = Vec::with_capacity(2 * cap - 1);
        lazy.resize(cap - 1, Op::id());
        lazy.extend(vals.iter().cloned().map(|x| x.into()));
        lazy.resize(2 * cap - 1, Op::id());
        DualSegmentTree {
            height: cap.trailing_zeros() as usize + 1,
            n,
            cap,
            lazy,
        }
    }
    fn apply(&mut self, idx: usize, p: &Op) {
        self.lazy[idx] = Op::op(p, &self.lazy[idx]);
    }
    fn push(&mut self, parent_idx: usize) {
        let left_idx = 2 * (parent_idx + 1) - 1;
        let right_idx = 2 * (parent_idx + 1);
        if left_idx < self.lazy.len() {
            let l = self.lazy[parent_idx].clone();
            self.apply(left_idx, &l);
            self.apply(right_idx, &l);
            self.lazy[parent_idx] = Op::id();
        }
    }
    fn push_all(&mut self, idx: usize) {
        for i in (1..(idx + 2).next_power_of_two().trailing_zeros()).rev() {
            self.push(((idx + 1) >> i) - 1);
        }
    }
    fn get(&mut self, pos: usize) -> Op {
        let idx = self.cap - 1 + pos;
        self.push_all(idx);
        self.lazy[idx].clone()
    }
    fn set<T>(&mut self, pos: usize, p: T)
    where
        T: Into<Op>,
    {
        let idx = self.cap - 1 + pos;
        self.push_all(idx);
        self.lazy[idx] = p.into();
    }
    fn update<T>(&mut self, r: impl std::ops::RangeBounds<usize>, p: T)
    where
        T: Into<Op>,
    {
        let (a, b) = range(r, self.n);
        let p = p.into();
        let mut left_idx = a + self.cap - 1;
        let mut right_idx = b + self.cap - 1;
        self.push_all(((left_idx + 1) >> (left_idx + 1).trailing_zeros()) - 1);
        self.push_all(((right_idx + 1) >> (right_idx + 1).trailing_zeros()) - 1);
        while left_idx < right_idx {
            if left_idx % 2 == 0 {
                self.apply(left_idx, &p);
            }
            if right_idx % 2 == 0 {
                self.apply(right_idx - 1, &p);
            }
            left_idx = left_idx >> 1;
            right_idx = (right_idx - 1) >> 1;
        }
    }
    fn dump_table(&self)
    where
        Op: std::fmt::Debug,
    {
        eprintln!("{}", segtree_table::<Op, Op>(&self.lazy, &[]));
    }
}
macro_rules! define_monoid {
    ($ name : ident , $ t : ty , $ id : expr , $ op : expr ) => {
        #[derive(Clone, Copy, PartialEq, Eq)]
        struct $name($t);
        impl Monoid for $name {
            fn id() -> Self {
                Self($id)
            }
            fn op(&self, rhs: &Self) -> Self {
                Self(($op)(self.0, rhs.0))
            }
        }
        impl From<$t> for $name {
            fn from(x: $t) -> $name {
                Self(x)
            }
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                if *self == Self::id() {
                    write!(f, "id.")
                } else {
                    self.0.fmt(f)
                }
            }
        }
    };
}

define_monoid!(
    Linear,
    (Mod998244353, Mod998244353),
    (Mod::one(), Mod::zero()),
    |lhs: (Mod998244353, Mod998244353), rhs: (Mod998244353, Mod998244353)| {
        // lhs.0 * (rhs.0 * x + rhs.1) + lhs.1
        (lhs.0 * rhs.0, lhs.0 * rhs.1 + lhs.1)
    }
);

fn main() {
    type Mod = Mod998244353;

    input! {
        n: usize, m: usize,
        a: [Mod; n],
        lrx: [(Usize1, usize, Mod); m],
    };

    let init = a.into_iter().map(|x| (Mod::zero(), x)).collect::<Vec<_>>();

    let mut st = lrx.into_iter().fold(
        DualSegmentTree::<Linear>::with(&init),
        |mut st, (l, r, x)| {
            let len = r - l;
            let p = 1 / Mod::new(len);

            st.update(l..r, (1 - p, p * x));

            st
        },
    );

    println!("{}", (0..n).map(|i| st.get(i).0 .1).join(" "));
}
