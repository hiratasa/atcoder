fn main() {
    let l = 12;
    let n = 4;

    // let d = 5;

    (0usize..1 << l)
        // .filter(|&s| s & 1 > 0)
        .filter(|&s| s.count_ones() == n)
        .map(|s| {
            (0..l)
                .filter(|&i| s & (1 << i) > 0)
                .tuple_combinations()
                .map(|(i, j)| min(j - i, l + i - j))
                .max()
                .unwrap()
        })
        .sorted()
        .dedup_with_count()
        .for_each(|(x, y)| {
            println!("# {x} {y}");
        });
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
