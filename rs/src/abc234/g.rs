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

trait Monoid {
    type Item: Clone;

    fn id() -> Self::Item;
    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item;
}

#[derive(Debug)]
struct SegmentTree<M>
where
    M: Monoid,
{
    cap: usize,
    values: Vec<M::Item>,
}

#[allow(dead_code)]
impl<M> SegmentTree<M>
where
    M: Monoid,
{
    fn new(n: usize) -> Self {
        let cap = n.next_power_of_two();
        SegmentTree {
            cap,
            values: vec![M::id(); 2 * cap - 1],
        }
    }

    fn with(vals: &Vec<M::Item>) -> Self {
        let n = vals.len();
        let cap = n.next_power_of_two();

        let mut values = Vec::with_capacity(2 * cap - 1);
        values.resize(cap - 1, M::id());
        values.extend(vals.iter().cloned());
        values.resize(2 * cap - 1, M::id());

        let mut st = SegmentTree { cap, values };
        for idx in (0..cap - 1).rev() {
            st.fix_value(idx);
        }
        st
    }

    fn fix_value(&mut self, idx: usize) {
        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);
        self.values[idx] = M::op(&self.values[left_idx], &self.values[right_idx]);
    }

    fn get(&self, pos: usize) -> M::Item {
        self.values[self.cap - 1 + pos].clone()
    }

    fn set(&mut self, pos: usize, v: M::Item) {
        let mut idx = self.cap - 1 + pos;

        self.values[idx] = v;

        while idx > 0 {
            idx = (idx - 1) / 2;
            self.fix_value(idx);
        }
    }

    fn query(&self, a: usize, b: usize) -> M::Item {
        let mut left = M::id();
        let mut right = M::id();

        let mut left_idx = a + self.cap - 1;
        let mut right_idx = b + self.cap - 1;

        let c0 = std::cmp::min(
            // trailing_ones
            (!left_idx).trailing_zeros(),
            (right_idx + 1).trailing_zeros(),
        );
        left_idx = left_idx >> c0;
        right_idx = ((right_idx + 1) >> c0) - 1;

        while left_idx < right_idx {
            if left_idx % 2 == 0 {
                left = M::op(&left, &self.values[left_idx]);
                left_idx += 1;
            }

            if right_idx % 2 == 0 {
                right = M::op(&self.values[right_idx - 1], &right);
                right_idx -= 1;
            }

            let c = std::cmp::min(
                // trailing_ones
                (!left_idx).trailing_zeros(),
                (right_idx + 1).trailing_zeros(),
            );
            left_idx = left_idx >> c;
            right_idx = ((right_idx + 1) >> c) - 1;
        }

        M::op(&left, &right)
    }

    // f(query(a, b)) == false となるbが存在すればその最小のものを返す
    // (存在しないときにnを返してしまうとquery(a,n)がfalseのときと区別がつかないのでNoneを返す)
    fn right_partition_point<F>(&self, a: usize, mut f: F) -> Option<usize>
    where
        F: FnMut(&M::Item) -> bool,
    {
        assert!(a <= self.cap);
        if !f(&M::id()) {
            Some(a)
        } else if a == self.cap {
            None
        } else {
            let mut b = a;
            // [b, b+2^k) が保持されている最初の箇所に移動
            let mut idx = ((b + self.cap) >> (b + self.cap).trailing_zeros()) - 1;
            let mut len = 1 << (b + self.cap).trailing_zeros();
            let mut val = M::id();
            let mut val_next = M::op(&val, &self.values[idx]);

            // チェックする範囲を拡大しながらf()がtrueになる限りbを右に伸ばしていく
            while f(&val_next) {
                val = val_next;
                b += len;

                // [b, b+2^k) が保持されている最初の箇所に移動
                len <<= (idx + 2).trailing_zeros();
                idx = ((idx + 2) >> (idx + 2).trailing_zeros()) - 1;

                // 最後に計算したidxが右端だった場合
                if idx == 0 {
                    return None;
                }
                val_next = M::op(&val, &self.values[idx]);
            }

            // 範囲を縮小しながらbを右に伸ばしていく
            idx = 2 * idx + 1;
            len >>= 1;
            while idx < self.values.len() {
                val_next = M::op(&val, &self.values[idx]);
                if f(&val_next) {
                    val = val_next;
                    b += len;
                    idx += 1;
                }
                len >>= 1;
                idx = 2 * idx + 1;
            }

            // [a, b)区間でfがtrue => 求めるbはその次
            Some(b + 1)
        }
    }

    // f(query(a, b)) == false となるaが存在すればその最大のもの+1を返す
    // 存在しない場合は0を返す
    fn left_partition_point<F>(&self, b: usize, mut f: F) -> usize
    where
        F: FnMut(&M::Item) -> bool,
    {
        assert!(b <= self.cap);
        if !f(&M::id()) {
            b
        } else if b == 0 {
            0
        } else {
            let mut a = b;
            // [a-2^k, a) が保持されている最初の箇所に移動
            let mut idx = (a + self.cap - 1) >> (!(a + self.cap - 1)).trailing_zeros();
            let mut len = 1 << (!(a + self.cap - 1)).trailing_zeros();
            if idx == 0 {
                // このケースになるのはb=self.capのときだけ
                len = self.cap;
            } else {
                idx -= 1;
            }

            let mut val = M::id();
            let mut val_next = M::op(&self.values[idx], &val);

            // チェックする範囲を拡大しながらf()がtrueになる限りaを左に伸ばしていく
            while f(&val_next) {
                val = val_next;
                a -= len;

                // 最後に計算したidxが左端だった場合
                if idx == 0 || (idx + 1).is_power_of_two() {
                    return 0;
                }

                // [a-2^k, a) が保持されている最初の箇所に移動
                len <<= (!idx).trailing_zeros();
                idx >>= (!idx).trailing_zeros();
                idx -= 1;

                val_next = M::op(&self.values[idx], &val);
            }

            // 範囲を縮小しながらaを左に伸ばしていく
            idx = 2 * idx + 2;
            len >>= 1;
            while idx < self.values.len() {
                val_next = M::op(&self.values[idx], &val);
                if f(&val_next) {
                    val = val_next;
                    a -= len;
                    idx -= 1;
                }
                len >>= 1;
                idx = 2 * idx + 2;
            }

            a
        }
    }
}

macro_rules! define_monoid {
    ($name: ident, $t: ty, $id: expr, $op: expr) => {
        #[derive(Clone, Debug)]
        struct $name;

        impl Monoid for $name {
            type Item = $t;

            fn id() -> Self::Item {
                $id
            }

            fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
                ($op)(*lhs, *rhs)
            }
        }
    };
}

define_monoid!(Sum, Mod998244353, Mod::zero(), std::ops::Add::add);

type ST = SegmentTree<Sum>;

fn main() {
    let n: usize = read();
    let a = read_row::<usize>();

    let (_, _, _, ans) = a.citer().enumerate().fold(
        (
            ST::with(&vvec![Mod::one(); Mod::zero(); n + 1]),
            vec![(usize::MAX, 0)],
            vec![(0, 0)],
            Mod::zero(),
        ),
        |(mut st, mut maxs, mut mins, mut s), (i, aa)| {
            maxs.push((aa, i + 1));
            while maxs[maxs.len() - 2].0 < aa {
                let j0 = maxs[maxs.len() - 2].1;
                let j1 = maxs[maxs.len() - 1].1;
                let x = maxs[maxs.len() - 2].0;

                s += Mod::new(aa - x) * st.query(j0 - 1, j1 - 1);
                maxs.remove(maxs.len() - 2);
                maxs.last_mut().unwrap().1 = j0;
            }

            mins.push((aa, i + 1));
            while mins[mins.len() - 2].0 > aa {
                let j0 = mins[mins.len() - 2].1;
                let j1 = mins[mins.len() - 1].1;
                let x = mins[mins.len() - 2].0;

                s += Mod::new(x - aa) * st.query(j0 - 1, j1 - 1);
                mins.remove(mins.len() - 2);
                mins.last_mut().unwrap().1 = j0;
            }

            st.set(i + 1, s);

            (st, maxs, mins, s)
        },
    );

    println!("{}", ans);
}
