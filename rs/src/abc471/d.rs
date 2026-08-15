fn main() {
    input! {
        q: usize, v: i64,
    };

    (0..q)
        .scan(BinaryHeap::new(), |q, _| {
            input! {
                ty: usize,
            };

            if ty == 1 {
                input! {
                    t: i64, w: i64,
                };

                q.push(w - t);

                Some(None)
            } else {
                input! {
                    t: i64,
                };

                if let Some(x) = q.pop() {
                    Some(Some((x + t).min(v)))
                } else {
                    Some(Some(-1))
                }
            }
        })
        .flatten()
        .for_each(|ans| {
            println!("{ans}");
        });
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
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
use rand::{Rng, SeedableRng, rngs::SmallRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
