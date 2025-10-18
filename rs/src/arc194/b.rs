fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    };

    let ans = p
        .into_iter()
        .enumerate()
        .scan(BIT::<Sum>::new(n), |bit, (i, x)| {
            let left_less = bit.sum(x);
            let left_greater = i - left_less;
            let right_less = x - left_less;

            bit.add(x, 1);

            let l = i + 1 - left_greater;
            let r = l + right_less;

            Some((l + r - 1) * (r - l) / 2)
        })
        .sum::<usize>();

    println!("{ans}");
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_with, successors},
    mem::{replace, swap, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};

trait Monoid {
    type Item: Clone;

    fn id() -> Self::Item;
    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item;
}

//  1-indexedで、
//    iの最後に立っているビットをB（=i&-i)として、
//    values_iは [i - (i&-i) + 1, i] の区間の和を保持
//    (といいつつ0-indexにアクセスする箇所で直してる)
// M must be commutative.
struct BIT<M>
where
    M: Monoid,
{
    len: usize,
    values: Vec<M::Item>,
}

#[allow(dead_code)]
impl<M> BIT<M>
where
    M: Monoid,
{
    fn new(len: usize) -> BIT<M> {
        BIT {
            len,
            values: vec![M::id(); len],
        }
    }

    fn with(vals: &Vec<M::Item>) -> Self {
        let mut bit = Self::new(vals.len());

        for (i, v) in vals.iter().enumerate() {
            bit.add(i, v.clone());
        }

        bit
    }

    // [0, i)の和
    fn sum(&self, i: usize) -> M::Item {
        let mut s = M::id();
        let mut idx = i as i64;

        // values[1] ~ values[i] の和
        // (bは1-indexedなのでこれでOK)
        while idx > 0 {
            s = M::op(&s, &self.values[(idx - 1) as usize]);
            idx -= idx & -idx;
        }

        return s;
    }

    fn add(&mut self, i: usize, a: M::Item) {
        // 1-indexedに直す
        let mut idx = i as i64 + 1;

        while idx as usize <= self.len {
            self.values[(idx - 1) as usize] = M::op(&self.values[(idx - 1) as usize], &a);
            idx += idx & -idx;
        }
    }

    fn len(&self) -> usize {
        self.len
    }
}

// 可換群
trait Group: Monoid {
    fn inv(value: &Self::Item) -> Self::Item;
}

#[allow(dead_code)]
impl<M> BIT<M>
where
    M: Group,
{
    // [i, j) の和
    fn sum_between(&self, i: usize, j: usize) -> M::Item {
        M::op(&self.sum(j), &M::inv(&self.sum(i)))
    }
}

macro_rules! define_monoid {
    ($name: ident, $t: ty, $id: expr, $op: expr) => {
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

define_monoid!(Sum, usize, 0, std::ops::Add::add);
