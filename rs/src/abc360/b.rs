fn main() {
    input! {
        s: Chars, t: Chars,
    };

    let ans = (1..=s.len())
        .tuple_combinations()
        .map(|(c, w)| (c, w - 1))
        .any(|(c, w)| {
            s.chunks(w)
                .into_iter()
                .filter_map(|ch| ch.get(c - 1))
                .copied()
                .collect::<Vec<_>>()
                == t
        });

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
