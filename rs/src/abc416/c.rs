fn main() {
    input! {
        n: usize, k: usize, x: usize,
        s: [Chars; n],
    };

    let ans = (0..k)
        .map(|_| 0..n)
        .multi_cartesian_product()
        .map(|v| {
            v.into_iter()
                .flat_map(|i| s[i].iter().copied())
                .collect::<String>()
        })
        .sorted()
        .nth(x - 1)
        .unwrap();

    print!("{ans}");
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
