fn main() {
    input! {
        n: usize,
        lr: [(i64, i64); 1 << n],
    };

    let mut prev = lr
        .iter()
        .copied()
        .map(|(l, r)| (vec![l, r], -r))
        .collect::<Vec<_>>();
    for i in 0..n {
        let mut next = vec![];

        for ((v0, x0), (v1, x1)) in prev.into_iter().tuples() {
            let mut v = v0.into_iter().merge(v1).collect::<Vec<_>>();
            let z0 = v.pop().unwrap();
            let z1 = v.pop().unwrap();
            v.push(z0);
            let x = x0 + x1 + z1;

            next.push((v, x));
        }

        prev = next;
    }

    let z = *prev[0].0.last().unwrap();
    let ans = z + prev[0].1;
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
