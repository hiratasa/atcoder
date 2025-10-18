fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m],
    };

    let ans = ab
        .into_iter()
        .sorted_by_key(|&(a, b)| a - b)
        .scan(n, |nn, (a, b)| {
            let k = (nn.saturating_sub(a - 1) + a - b - 1) / (a - b);

            *nn -= k * (a - b);

            Some(k)
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
