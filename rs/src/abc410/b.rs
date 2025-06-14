fn main() {
    input! {
        n: usize, q: usize,
        x: [usize; q],
    };

    println!(
        "{}",
        x.iter()
            .scan(vec![0; n], |t, &c| {
                if c >= 1 {
                    t[c - 1] += 1;
                    Some(c)
                } else {
                    let mi = t.iter().position_min().unwrap();
                    t[mi] += 1;
                    Some(mi + 1)
                }
            })
            .join(" ")
    );
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
