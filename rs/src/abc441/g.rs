fn main() {
    input! {
        n: usize, q: usize,
    };

    let mut st = LazySegmentTree::<Dish, Operation>::with(&vec![(Some(0), false); n]);
    for _ in 0..q {
        input! {
            ty: usize,
        };

        if ty == 1 {
            input! {
                l: Usize1, r: usize, x: usize,
            };

            st.update(l..r, (x, None));
        } else if ty == 2 {
            input! {
                l: Usize1, r: usize,
            };

            st.update(l..r, (0, Some(true)));
        } else {
            input! {
                l: Usize1, r: usize,
            };

            let a = st.query(l..r);

            println!("{}", a.0.0.unwrap_or(0));
        }
    }
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_n, repeat_with, successors},
    mem::{replace, swap, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, iterate, izip};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};

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
struct LazySegmentTree<M, Op>
where
    M: Monoid,
    Op: Monoid,
{
    #[allow(dead_code)]
    height: usize,
    n: usize,
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
            n,
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
            n,
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
        self.fix_all(a + self.cap - 1);
        self.fix_all(b + self.cap - 1);
    }
    fn query(&mut self, r: impl std::ops::RangeBounds<usize>) -> M {
        let (a, b) = range(r, self.n);
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
    fn dump_table(&self)
    where
        M: std::fmt::Debug,
        Op: std::fmt::Debug,
    {
        eprintln!("{}", segtree_table::<M, Op>(&self.values, &self.lazy));
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

define_monoid!(Dish, (Option<usize>, bool), (None, false), |lhs: (
    Option<usize>,
    bool
),
                                                            rhs: (
    Option<usize>,
    bool
)| {
    (max(lhs.0, rhs.0), lhs.1 || rhs.1)
});

define_monoid!(Operation, (usize, Option<bool>), (0, None), |lhs: (
    usize,
    Option<bool>
),
                                                             rhs: (
    usize,
    Option<bool>
)| {
    match (lhs, rhs) {
        ((l, None), (r, None)) => (l + r, None),
        ((l, None), (r, Some(x))) => (l + r, Some(x)),
        ((l, Some(x)), (_r, None)) => (l, Some(x)),
        ((l, Some(x)), (_r, Some(y))) => (l, Some(x ^ y)),
    }
});

impl Operator<Dish> for Operation {
    fn apply(&self, v: &Dish) -> Dish {
        let l = self.0.0;

        match (self.0.1, v.0.0, v.0.1) {
            (None, v, x) => Dish((v.map(|r| l + r), x)),
            (Some(true), v, x) => Dish((x.then_some(l), v.is_some())),
            (Some(false), v, x) => Dish((v.map(|_| l), x)),
        }
    }
}
