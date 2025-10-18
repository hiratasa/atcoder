fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let ans = a
        .into_iter()
        .enumerate()
        .scan(FxHashMap::default(), |map, (i, x)| {
            let c = map.get(&(i as i64 - x)).copied().unwrap_or(0usize);
            *map.entry(i as i64 + x).or_insert(0usize) += 1;

            Some(c)
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
