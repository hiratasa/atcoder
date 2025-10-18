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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
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
#[derive(Debug)]
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
            height: cap.trailing_zeros() as usize + 1,
            cap,
            values: vec![M::id(); 2 * cap - 1],
            lazy: vec![Op::id(); 2 * cap - 1],
        }
    }

    fn with(vals: &Vec<M::Item>) -> Self {
        let n = vals.len();
        let cap = n.next_power_of_two();

        let mut values = Vec::with_capacity(2 * cap - 1);
        values.resize(cap - 1, M::id());
        values.extend(vals.iter().cloned());
        values.resize(2 * cap - 1, M::id());

        let mut st = LazySegmentTree {
            height: cap.trailing_zeros() as usize + 1,
            cap,
            values,
            lazy: vec![Op::id(); 2 * cap - 1],
        };

        for idx in (0..cap - 1).rev() {
            st.fix_value(idx);
        }

        st
    }

    // internal
    fn get_node_value(&mut self, idx: usize) -> M::Item {
        Op::apply(&self.lazy[idx], &self.values[idx])
    }

    // internal
    fn fix_value(&mut self, idx: usize) {
        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);
        if left_idx < self.values.len() {
            self.values[idx] = M::op(
                &self.get_node_value(left_idx),
                &self.get_node_value(right_idx),
            );
        }
    }

    // internal
    fn resolve(&mut self, parent_idx: usize) {
        let left_idx = 2 * (parent_idx + 1) - 1;
        let right_idx = 2 * (parent_idx + 1);

        if left_idx < self.values.len() {
            self.lazy[left_idx] = Op::op(&self.lazy[parent_idx], &self.lazy[left_idx]);
            self.lazy[right_idx] = Op::op(&self.lazy[parent_idx], &self.lazy[right_idx]);
            self.lazy[parent_idx] = Op::id();
            self.fix_value(parent_idx);
        } else {
            self.values[parent_idx] = Op::apply(&self.lazy[parent_idx], &self.values[parent_idx]);
            self.lazy[parent_idx] = Op::id();
        }
    }

    // internal
    fn resolve_all(&mut self, pos: usize) {
        let idx = self.cap - 1 + pos;
        for i in (0..self.height).rev() {
            self.resolve(((idx + 1) >> i) - 1);
        }
    }

    fn get(&mut self, pos: usize) -> M::Item {
        self.resolve_all(pos);

        let idx = self.cap - 1 + pos;
        self.values[idx].clone()
    }

    fn set(&mut self, pos: usize, v: M::Item) {
        self.resolve_all(pos);

        let mut idx = self.cap - 1 + pos;
        self.values[idx] = v;
        self.lazy[idx] = Op::id();

        while idx > 0 {
            idx = (idx - 1) / 2;
            self.fix_value(idx);
        }
    }

    fn update(&mut self, a: usize, b: usize, p: Op::Item) {
        let mut left_idx = a + self.cap - 1;
        let mut right_idx = b + self.cap - 1;

        // Opが非可換の場合用に, これより前にupdateされたものを適用させておく
        for i in (1..self.height).rev() {
            self.resolve(((left_idx + 1) >> i) - 1);
            self.resolve(((right_idx + 1) >> i) - 1);
        }

        while left_idx < right_idx {
            if left_idx % 2 == 0 {
                self.lazy[left_idx] = Op::op(&p, &self.lazy[left_idx]);
            }

            if right_idx % 2 == 0 {
                self.lazy[right_idx - 1] = Op::op(&p, &self.lazy[right_idx - 1]);
            }

            // 偶数の場合は一つ右隣の親になる
            left_idx = left_idx >> 1;
            right_idx = (right_idx - 1) >> 1;
        }

        let mut left_idx = a + self.cap - 1;
        let mut right_idx = b + self.cap - 1;
        for _ in 0..self.height - 1 {
            left_idx = (left_idx - 1) >> 1;
            self.fix_value(left_idx);

            right_idx = (right_idx - 1) >> 1;
            // This is out of bounds if b == self.cap.
            // (currently checked in fix_value())
            self.fix_value(right_idx);
        }
    }

    fn query(&mut self, a: usize, b: usize) -> M::Item {
        let mut left = M::id();
        let mut right = M::id();

        let mut left_idx = a + self.cap - 1;
        let mut right_idx = b + self.cap - 1;

        let c0 = std::cmp::min(
            // trailing_ones
            (!left_idx).trailing_zeros(),
            (right_idx + 1).trailing_zeros(),
        ) as usize;

        for i in (c0 + 1..self.height).rev() {
            self.resolve(((left_idx + 1) >> i) - 1);
            self.resolve(((right_idx + 1) >> i) - 1);
        }

        left_idx = left_idx >> c0;
        right_idx = ((right_idx + 1) >> c0) - 1;

        while left_idx < right_idx {
            if left_idx % 2 == 0 {
                left = M::op(&left, &self.get_node_value(left_idx));
                left_idx += 1;
            }

            if right_idx % 2 == 0 {
                right = M::op(&self.get_node_value(right_idx - 1), &right);
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
}

#[derive(Clone, Copy, Debug)]
struct Sum;

impl Monoid for Sum {
    type Item = (usize, usize);

    fn id() -> Self::Item {
        (0, 0)
    }

    fn op(&lhs: &Self::Item, &rhs: &Self::Item) -> Self::Item {
        (lhs.0 + rhs.0, lhs.1 + rhs.1)
    }
}

#[derive(Clone, Copy, Debug)]
struct Update;

impl Monoid for Update {
    type Item = Option<usize>;

    fn id() -> Self::Item {
        None
    }

    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
        lhs.or(*rhs)
    }
}

impl Operator<(usize, usize)> for Update {
    fn apply(op: &Self::Item, &v: &(usize, usize)) -> (usize, usize) {
        op.map_or(v, |x| (x * v.1, v.1))
    }
}

fn main() {
    let (n, q) = read_tuple!(usize, usize);
    let a = read_row::<usize>();
    let query = read_vec(q, || read::<String>());

    query
        .iter()
        .scan(
            (
                LazySegmentTree::<Sum, Update>::with(
                    &a.citer().map(|x| (x, 1)).collect::<Vec<_>>(),
                ),
                (0..n).map(|i| (i, i + 1)).collect::<BTreeSet<_>>(),
            ),
            |(st, non_zeros), query| {
                let mut it = query.split_whitespace();
                let t = it.next().unwrap().parse::<usize>().unwrap();
                let l = it.next().unwrap().parse::<usize>().unwrap() - 1;
                let r = it.next().unwrap().parse::<usize>().unwrap();

                let split = |non_zeros: &mut BTreeSet<(usize, usize)>, i: usize| {
                    if let Some(&(l, r)) = non_zeros.range(..(i, usize::MAX)).next_back() {
                        if l <= i && i < r {
                            non_zeros.remove(&(l, r));

                            if l != i {
                                non_zeros.insert((l, i));
                            }
                            non_zeros.insert((i, r));
                        }
                    }
                };

                // eprintln!("{:?}", st);

                match t {
                    1 => {
                        let x = it.next().unwrap().parse::<usize>().unwrap();

                        split(non_zeros, l);
                        split(non_zeros, r);

                        let idxs = non_zeros.range((l, 0)..(r, 0)).copied().collect::<Vec<_>>();

                        for (ll, rr) in idxs {
                            let v = st.get(ll).0;
                            let u = v / x;

                            st.update(ll, rr, Some(u));

                            if u == 0 {
                                non_zeros.remove(&(ll, rr));
                            }
                        }

                        Some(None)
                    }
                    2 => {
                        let y = it.next().unwrap().parse::<usize>().unwrap();

                        split(non_zeros, l);
                        split(non_zeros, r);

                        let idxs = non_zeros.range((l, 0)..(r, 0)).copied().collect::<Vec<_>>();

                        for (ll, rr) in idxs {
                            non_zeros.remove(&(ll, rr));
                        }

                        st.update(l, r, Some(y));
                        non_zeros.insert((l, r));

                        Some(None)
                    }
                    3 => Some(Some(st.query(l, r).0)),
                    _ => unreachable!(),
                }
            },
        )
        .flatten()
        .for_each(|ans| {
            println!("{}", ans);
        });
}
