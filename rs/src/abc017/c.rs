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

trait Monoid {
    type Item: Clone;

    fn id() -> Self::Item;
    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item;
}

// M: Monoid of value
// Op: Monoid of lazy operation
struct LazySegmentTree<M, Op>
where
    M: Monoid,
    Op: Monoid,
{
    height: usize,
    cap: usize,
    values: Vec<M::Item>,
    lazy: Vec<Op::Item>,
}

trait Operator<T>: Monoid {
    fn apply(op: &Self::Item, v: &T) -> T;
}

#[allow(dead_code)]
impl<M, Op> LazySegmentTree<M, Op>
where
    M: Monoid,
    Op: Monoid + Operator<M::Item>,
{
    fn new(n: usize) -> Self {
        let cap = n.next_power_of_two();
        LazySegmentTree {
            height: cap.trailing_zeros() as usize,
            cap,
            values: vec![M::id(); 2 * cap - 1],
            lazy: vec![Op::id(); 2 * cap - 1],
        }
    }

    fn get_node_value(&mut self, idx: usize) -> M::Item {
        Op::apply(&self.lazy[idx], &self.values[idx])
    }

    fn fix_value(&mut self, idx: usize) {
        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);
        self.values[idx] = M::op(
            &self.get_node_value(left_idx),
            &self.get_node_value(right_idx),
        );
    }

    fn resolve(&mut self, pos: usize) {
        let idx = self.cap - 1 + pos;
        for i in (1..self.height).rev() {
            let parent_idx = ((idx + 1) >> i) - 1;

            let left_idx = 2 * (parent_idx + 1) - 1;
            let right_idx = 2 * (parent_idx + 1);

            self.lazy[left_idx] = Op::op(&self.lazy[parent_idx], &self.lazy[left_idx]);
            self.lazy[right_idx] = Op::op(&self.lazy[parent_idx], &self.lazy[right_idx]);
            self.lazy[parent_idx] = Op::id();

            self.fix_value(parent_idx);
        }

        self.values[idx] = Op::apply(&self.lazy[idx], &self.values[idx]);
        self.lazy[idx] = Op::id();
    }

    fn get(&mut self, pos: usize) -> M::Item {
        self.resolve(pos);

        let idx = self.cap - 1 + pos;
        self.values[idx].clone()
    }

    fn set(&mut self, pos: usize, v: M::Item) {
        self.resolve(pos);

        let mut idx = self.cap - 1 + pos;
        self.values[idx] = v;
        self.lazy[idx] = Op::id();

        while idx > 0 {
            idx = (idx - 1) / 2;
            self.fix_value(idx);
        }
    }

    fn update(&mut self, a: usize, b: usize, p: Op::Item) {
        self.update_impl(a, b, p, 0, 0, self.cap);
    }

    fn update_impl(&mut self, a: usize, b: usize, p: Op::Item, idx: usize, l: usize, r: usize) {
        if a >= r || b <= l {
            // no overlap
            return;
        }

        if a <= l && r <= b {
            self.lazy[idx] = Op::op(&p, &self.lazy[idx]);
            return;
        }

        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);

        // モノイドOpが可換でない場合、pの適用前にlazy[idx]の適用が必要
        self.lazy[left_idx] = Op::op(&self.lazy[idx], &self.lazy[left_idx]);
        self.lazy[right_idx] = Op::op(&self.lazy[idx], &self.lazy[right_idx]);
        self.lazy[idx] = Op::id();

        self.update_impl(a, b, p.clone(), left_idx, l, (l + r) / 2);
        self.update_impl(a, b, p.clone(), right_idx, (l + r) / 2, r);

        self.fix_value(idx);
    }

    fn query(&mut self, a: usize, b: usize) -> M::Item {
        self.query_impl(a, b, 0, 0, self.cap)
    }

    fn query_impl(&mut self, a: usize, b: usize, idx: usize, l: usize, r: usize) -> M::Item {
        if a >= r || b <= l {
            // no overlap
            return M::id();
        }

        if a <= l && r <= b {
            return self.get_node_value(idx);
        }

        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);

        let left_v = self.query_impl(a, b, left_idx, l, (l + r) / 2);
        let right_v = self.query_impl(a, b, right_idx, (l + r) / 2, r);

        Op::apply(&self.lazy[idx], &M::op(&left_v, &right_v))
    }
}

macro_rules! define_monoid {
    ($name: ident, $t: ty, $id: expr, $op: expr) => {
        #[derive(Clone)]
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

define_monoid!(Maximum, usize, 0, usize::max);
define_monoid!(Addition, usize, 0, std::ops::Add::add);

impl Operator<usize> for Addition {
    fn apply(op: &Self::Item, v: &usize) -> usize {
        *op + *v
    }
}

type ST = LazySegmentTree<Maximum, Addition>;

fn main() {
    let (n, m) = read_tuple!(usize, usize);

    let lrs = read_vec(n, || read_tuple!(usize, usize, usize));

    let (mut st, ans0) = lrs
        .citer()
        .map(|(l, r, s)| (l - 1, r - 1, s))
        .sorted_by_key(|&(l, _r, _s)| l)
        .fold(
            (ST::new(m + 1), 0usize),
            |(mut st_all, p_noall), (l, r, s)| {
                let p_noall = if l == 0 {
                    0
                } else {
                    max(st_all.query(0, l), p_noall) + s
                };

                let p_all = st_all.query(l, r + 2) + s;
                st_all.set(r + 1, p_all);
                st_all.update(r + 2, m + 1, s);

                (st_all, p_noall)
            },
        );
    let ans = max(st.query(0, m), ans0);
    println!("{}", ans);
}
