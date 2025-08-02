fn main() {
    input! {
        s: Chars,
    };

    let ans = (0..s.len())
        .map(|i| {
            if s[i] == '#' {
                '#'
            } else if i + 1 == s.len() {
                'o'
            } else if s[i + 1] == '#' {
                'o'
            } else {
                '.'
            }
        })
        .collect::<String>();

    println!("{ans}");
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
