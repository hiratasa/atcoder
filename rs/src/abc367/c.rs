fn main() {
    input! {
        n: usize, k: usize,
        r: [usize; n],
    };

    (0..n)
        .map(|i| 1..=r[i])
        .multi_cartesian_product()
        .filter(|v| v.iter().sum::<usize>() % k == 0)
        .for_each(|v| {
            println!("{}", v.iter().join(" "));
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
