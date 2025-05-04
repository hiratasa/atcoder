fn main() {
    input! {
        n: usize,
        cases: [[usize]; n],
    };

    cases
        .into_iter()
        .map(|a| {
            a.into_iter()
                .sorted()
                .enumerate()
                .scan((VecDeque::new(), 0), |(q, s), (i, x)| {
                    q.push_back(x);
                    *s += x;

                    while matches!(q.front(), Some(&y) if (i + 1) * y <= *s) {
                        q.pop_front();
                    }

                    Some(q.len())
                })
                .max()
                .unwrap()
        })
        .for_each(|ans| {
            println!("{ans}");
        })
}

#[allow(unused_imports)]
use std::{
    cmp::{max, min, Ordering, Reverse},
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
