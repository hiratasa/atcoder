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
#[allow(unused_imports)]
use rand::{rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom, Rng, SeedableRng};
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
fn read_digits() -> Vec<usize> {
    read::<String>()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
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
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
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

trait Monoid {
    fn id() -> Self;
    fn op(lhs: &Self, rhs: &Self) -> Self;
}
#[derive(Debug)]
struct LazySegmentTree<M, Op>
where
    M: Monoid,
    Op: Monoid,
{
    height: usize,
    cap: usize,
    values: Vec<M>,
    lazy: Vec<Op>,
}
trait Operator<T>: Monoid {
    fn apply(&self, v: &T) -> T;
}
#[allow(dead_code)]
impl<M, Op> LazySegmentTree<M, Op>
where
    M: Monoid + Clone,
    Op: Monoid + Operator<M> + Clone,
{
    fn new(n: usize) -> Self {
        let cap = n.next_power_of_two();
        LazySegmentTree {
            height: cap.trailing_zeros() as usize + 1,
            cap,
            values: vec![M::id(); 2 * cap - 1],
            lazy: vec![Op::id(); 2 * cap - 1],
        }
    }
    fn with<T>(vals: &[T]) -> Self
    where
        T: Into<M> + Clone,
    {
        let n = vals.len();
        let cap = n.next_power_of_two();
        let mut values = Vec::with_capacity(2 * cap - 1);
        values.resize(cap - 1, M::id());
        values.extend(vals.iter().cloned().map(|x| x.into()));
        values.resize(2 * cap - 1, M::id());
        let mut st = LazySegmentTree {
            height: cap.trailing_zeros() as usize + 1,
            cap,
            values,
            lazy: vec![Op::id(); 2 * cap - 1],
        };
        for idx in (0..cap - 1).rev() {
            st.fix(idx);
        }
        st
    }
    fn fix(&mut self, idx: usize) {
        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);
        if left_idx < self.values.len() {
            self.values[idx] = Op::apply(
                &self.lazy[idx],
                &M::op(&self.values[left_idx], &self.values[right_idx]),
            );
        }
    }
    fn fix_all(&mut self, mut idx: usize) {
        while idx > 0 {
            idx = (idx - 1) / 2;
            self.fix(idx);
        }
    }
    fn apply(&mut self, idx: usize, p: &Op) {
        self.lazy[idx] = Op::op(p, &self.lazy[idx]);
        self.values[idx] = Op::apply(p, &self.values[idx]);
    }
    fn push(&mut self, parent_idx: usize) {
        let left_idx = 2 * (parent_idx + 1) - 1;
        let right_idx = 2 * (parent_idx + 1);
        if left_idx < self.values.len() {
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
    fn get(&mut self, pos: usize) -> M {
        let idx = self.cap - 1 + pos;
        self.push_all(idx);
        self.values[idx].clone()
    }
    fn set<T>(&mut self, pos: usize, v: T)
    where
        T: Into<M>,
    {
        let idx = self.cap - 1 + pos;
        self.push_all(idx);
        self.values[idx] = v.into();
        self.lazy[idx] = Op::id();
        self.fix_all(idx);
    }
    fn update<T>(&mut self, a: usize, b: usize, p: T)
    where
        T: Into<Op>,
    {
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
        self.fix_all(a + self.cap - 1);
        self.fix_all(b + self.cap - 1);
    }
    fn query(&mut self, a: usize, b: usize) -> M {
        let mut left = M::id();
        let mut right = M::id();
        let mut left_idx = a + self.cap - 1;
        let mut right_idx = b + self.cap - 1;
        self.push_all(((left_idx + 1) >> (left_idx + 1).trailing_zeros()) - 1);
        self.push_all(((right_idx + 1) >> (right_idx + 1).trailing_zeros()) - 1);
        while left_idx < right_idx {
            if left_idx % 2 == 0 {
                left = M::op(&left, &self.values[left_idx]);
                left_idx += 1;
            }
            if right_idx % 2 == 0 {
                right = M::op(&self.values[right_idx - 1], &right);
                right_idx -= 1;
            }
            left_idx = left_idx >> 1;
            right_idx = (right_idx - 1) >> 1;
        }
        M::op(&left, &right)
    }
}
macro_rules! define_monoid {
    ($ name : ident , $ t : ty , $ id : expr , $ op : expr ) => {
        #[derive(Clone, Copy, Debug)]
        struct $name($t);
        impl Monoid for $name {
            fn id() -> Self {
                Self($id)
            }
            fn op(lhs: &Self, rhs: &Self) -> Self {
                Self(($op)(lhs.0, rhs.0))
            }
        }
        impl From<$t> for $name {
            fn from(x: $t) -> $name {
                Self(x)
            }
        }
    };
}

define_monoid!(Min, usize, usize::MAX, std::cmp::min);
define_monoid!(Sub, usize, 0, std::ops::Add::add);

impl Operator<Min> for Sub {
    fn apply(&self, v: &Min) -> Min {
        Min(v.0 - self.0)
    }
}

fn main() {
    let n = read::<usize>();

    let mut ab = read_vec(n, || read_tuple!(usize, usize));

    ab.sort();

    let a = ab.citer().map(|(a, _)| a).collect::<Vec<_>>();
    let b = ab.citer().map(|(_, b)| b).sorted().collect::<Vec<_>>();

    if izip!(a.citer(), b.citer()).any(|(x, y)| x < y) {
        println!("-1");
        return;
    }

    let t = a
        .citer()
        .scan(0, |idx, x| {
            while *idx + 1 < n && x >= b[*idx + 1] {
                *idx += 1;
            }

            Some(*idx)
        })
        .collect::<Vec<_>>();

    let (_, ans) = ab
        .citer()
        .enumerate()
        .filter(|&(_, (x, y))| x >= y)
        .map(|(i, (_x, y))| {
            let idx = t
                .binary_search_by(|&idx| b[idx].cmp(&y).then(Ordering::Greater))
                .unwrap_err();

            (min(idx, i), i)
        })
        .sorted_by_key(|&(_i, j)| j)
        .fold(
            (
                LazySegmentTree::<Min, Sub>::with(
                    &t.citer()
                        .enumerate()
                        .map(|(i, j)| j - i)
                        .collect::<Vec<_>>(),
                ),
                0,
            ),
            |(mut st, num), (i, j)| {
                if st.query(i, j).0 > 0 {
                    st.update(i, j, 1);
                    (st, num + 1)
                } else {
                    (st, num)
                }
            },
        );

    println!("{}", ans);
}
