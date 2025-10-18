fn main() {
    input! {
        n: usize, m: usize, k: usize,
        results: [([Usize1], char); m],
    };

    let ans = (0usize..1 << n)
        .filter(|&s| {
            results
                .iter()
                .all(|(a, r)| (a.iter().filter(|&&i| s & (1 << i) > 0).count() >= k) == (*r == 'o'))
        })
        .count();

    println!("{ans}");
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::*,
    mem::{replace, take},
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
