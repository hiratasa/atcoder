fn main() {
    input! {
        s: [Chars]
    };

    let t = s
        .iter()
        .tuple_combinations()
        .flat_map(|(s, t)| {
            [
                s.iter().chain(t).copied().collect::<String>(),
                t.iter().chain(s).copied().collect::<String>(),
            ]
        })
        .sorted()
        .dedup()
        .collect::<Vec<_>>();

    println!("{}", t.len());
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
