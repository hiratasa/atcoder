fn main() {
    input! {
        p: [[i64; 2]; 3],
    };

    let ans = p
        .iter()
        .tuple_combinations()
        .map(|(p0, p1)| (p0[0] - p1[0]).pow(2) + (p0[1] - p1[1]).pow(2))
        .sorted()
        .zip([1, 1, -1])
        .map(|(d, s)| d * s)
        .sum::<i64>()
        == 0;

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
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
