fn main() {
    input! {
        n: usize, m: usize,
        s: [Chars; n],
    };

    let ans = iproduct!(0..=n - m, 0..=n - m)
        .map(|(i, j)| {
            s[i..i + m]
                .iter()
                .map(|row| row[j..j + m].to_vec())
                .collect::<Vec<_>>()
        })
        .unique()
        .count();

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
use itertools::{Itertools, chain, iproduct, iterate, izip};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
