fn main() {
    input! {
        n: usize, m: usize, k: usize,
        events: [(Usize1, Usize1); k],
    };

    let ans = events
        .into_iter()
        .scan(vec![0; n], |counts, (a, _b)| {
            counts[a] += 1;

            if counts[a] == m {
                Some(Some(a))
            } else {
                Some(None)
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    println!("{}", ans.iter().map(|&i| i + 1).join(" "));
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
