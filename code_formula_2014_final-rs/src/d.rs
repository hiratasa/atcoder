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

trait IteratorExt: Iterator + Sized {
    fn fold_vec<T, F>(self: Self, init: Vec<T>, f: F) -> Vec<T>
    where
        F: FnMut(Self::Item) -> (usize, T);
    fn fold_vec2<T, F>(self: Self, init: Vec<T>, f: F) -> Vec<T>
    where
        F: FnMut(&Vec<T>, Self::Item) -> (usize, T);
    fn fold_vec3<T, F>(self: Self, init: Vec<T>, f: F) -> Vec<T>
    where
        F: FnMut(&Vec<T>, Self::Item) -> T;
}
impl<I> IteratorExt for I
where
    I: Iterator,
{
    fn fold_vec<T, F>(self: Self, init: Vec<T>, mut f: F) -> Vec<T>
    where
        F: FnMut(Self::Item) -> (usize, T),
    {
        self.fold(init, |mut v, item| {
            let (idx, t) = f(item);
            v[idx] = t;
            v
        })
    }
    fn fold_vec2<T, F>(self: Self, init: Vec<T>, mut f: F) -> Vec<T>
    where
        F: FnMut(&Vec<T>, Self::Item) -> (usize, T),
    {
        self.fold(init, |mut v, item| {
            let (idx, t) = f(&v, item);
            v[idx] = t;
            v
        })
    }
    fn fold_vec3<T, F>(self: Self, init: Vec<T>, mut f: F) -> Vec<T>
    where
        F: FnMut(&Vec<T>, Self::Item) -> T,
    {
        self.fold(init, |mut v, item| {
            let t = f(&v, item);
            v.push(t);
            v
        })
    }
}

trait Monoid {
    type Item: Clone;

    fn id() -> Self::Item;
    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item;
}

#[derive(Clone)]
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
        self.query_impl(a, b, 0, 0, self.cap)
    }

    fn query_impl(&self, a: usize, b: usize, idx: usize, l: usize, r: usize) -> M::Item {
        if a >= r || b <= l {
            // no overlap
            return M::id();
        }

        if a <= l && r <= b {
            return self.values[idx].clone();
        }

        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);

        let left_v = self.query_impl(a, b, left_idx, l, (l + r) / 2);
        let right_v = self.query_impl(a, b, right_idx, (l + r) / 2, r);

        M::op(&left_v, &right_v)
    }

    // f(query(a, b)) != false となるbが存在すればその最小のものを返す
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
            self.right_partition_point_impl(a, &mut f, 0, 0, self.cap, &M::id())
                .ok()
        }
    }

    fn right_partition_point_impl<F>(
        &self,
        a: usize,
        f: &mut F,
        idx: usize,
        l: usize,
        r: usize,
        carry: &M::Item,
    ) -> Result<usize, M::Item>
    where
        F: FnMut(&M::Item) -> bool,
    {
        // precondition
        assert!(a < r);
        assert!(f(carry));
        // assert!(l < a || carry == query(a, l))

        // postcondition
        // when return Ok, f(query(a, ok_value - 1)) && !f(query(a, ok_value))
        // when return Err, err_value == query(a, r) && f(err_value)

        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);
        let mid = (l + r) / 2;

        if a <= l {
            let v = M::op(carry, &self.values[idx]);
            if f(&v) {
                Err(v.clone())
            } else {
                if l + 1 == r {
                    // leaf
                    Ok(r)
                } else {
                    match self.right_partition_point_impl(a, f, left_idx, l, mid, carry) {
                        Ok(found) => Ok(found),
                        // In this branch, expected to be always return Ok
                        Err(q) => self.right_partition_point_impl(a, f, right_idx, mid, r, &q),
                    }
                }
            }
        } else if a < mid {
            match self.right_partition_point_impl(a, f, left_idx, l, mid, &M::id()) {
                Ok(found) => Ok(found),
                Err(q) => self.right_partition_point_impl(a, f, right_idx, mid, r, &q),
            }
        } else if a < r {
            self.right_partition_point_impl(a, f, right_idx, mid, r, &M::id())
        } else {
            unreachable!()
        }
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

type ST = SegmentTree<Maximum>;

fn main() {
    let n: usize = read();

    let h = read_row::<usize>();

    let mse = read_vec(n, || read_tuple!(usize, usize, usize));

    let t = mse
        .citer()
        .map(|(m, s, e)| (m, (s, e)))
        .into_group_map()
        .into_iter()
        .flat_map(|(_m, v)| {
            v.citer()
                .sorted()
                .fold_vec3(
                    vec![],
                    |d: &Vec<Vec<(usize, usize, usize, usize)>>, (s, e)| {
                        if let Some(i) = d
                            .iter()
                            .map(|dd| dd[0])
                            .rposition(|(_s2, e2, _, _)| e2 <= s)
                        {
                            chain(
                                once((s, e, h[0], 1)),
                                d[i].citer()
                                    .map(|(s2, _e2, hh, k)| (s2, e, hh + h[k], k + 1)),
                            )
                            .collect_vec()
                        } else {
                            vec![(s, e, h[0], 1)]
                        }
                    },
                )
                .into_iter()
                .flatten()
        });

    let st =
        t.sorted_by_key(|&(_s, e, _hh, _)| e)
            .fold(ST::new(100001), |mut st, (s, e, hh, _)| {
                let p = st.query(0, s + 1) + hh;
                let q = st.get(e);

                if p > q {
                    st.set(e, p);
                }

                st
            });
    let ans = st.query(0, 100001);

    println!("{}", ans);
}
