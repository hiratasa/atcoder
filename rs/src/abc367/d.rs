fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
    };

    let sum = a.iter().sum::<usize>();

    let ans = a
        .iter()
        .scan((0, FxHashMap::default()), |(csum, map), x| {
            *csum += x;
            let r = *csum % m;

            let b0 = map.get(&r).copied().unwrap_or_default();
            let b1 = map
                .get(&((m + r - sum % m) % m))
                .copied()
                .unwrap_or_default();
            *map.entry(r).or_insert(0) += 1;

            Some(b0 + b1)
        })
        .sum::<usize>();

    println!("{ans}");
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
