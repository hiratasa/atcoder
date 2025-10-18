fn main() {
    input! {
        a: [usize],
    };

    println!(
        "{}",
        a.into_iter()
            .enumerate()
            .scan(FxHashMap::default(), |prevs, (i, x)| {
                let r = prevs.get(&x).copied();
                prevs.insert(x, i);
                Some(r)
            })
            .map(|ans| { ans.map_or(-1, |i| (i + 1) as i64) })
            .join(" ")
    )
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
