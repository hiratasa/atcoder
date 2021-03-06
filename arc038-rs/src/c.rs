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

trait Monoid {
    type Item: Clone;

    fn id() -> Self::Item;
    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item;
}

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

    // f(query(a, b)) == false となるbが存在すればその最小のものを返す
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

define_monoid!(
    Minimum,
    (usize, usize),
    (usize::MAX, usize::MAX),
    std::cmp::min
);

type ST = SegmentTree<Minimum>;

fn main() {
    let n: usize = read();
    let ca = read_vec(n - 1, || read_tuple!(usize, usize));

    let grundy = (0..n - 1)
        .scan(
            (ST::with(&vvec![(0, 0); Minimum::id(); n]), 1),
            |(st, next), i| {
                let m = st.query(0, i + 1 - ca[i].0);
                let g = if m == Minimum::id() {
                    *next
                } else {
                    st.set(m.1, Minimum::id());
                    m.0
                };

                st.set(i + 1, (g, i + 1));
                *next = max(*next, g + 1);

                Some(g)
            },
        )
        .collect_vec();

    let g = izip!(ca.citer().map(|t| t.1), grundy.citer())
        .map(|(a, g)| (a % 2) * g)
        .fold(0usize, |acc, g| acc ^ g);
    if g == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
