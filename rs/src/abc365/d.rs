fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    const HANDS: &str = "RPS";

    let dp = s
        .into_iter()
        .map(|c| HANDS.chars().position(|d| d == c).unwrap())
        .fold(vec![Some(0); 3], |prev, c| {
            (0..3)
                .map(|i| {
                    if (i + 1) % 3 != c {
                        let win = i == (c + 1) % 3;
                        (0..3)
                            .filter(|&j| j != i)
                            .filter_map(|j| prev[j].map(|x| x + win as usize))
                            .max()
                    } else {
                        None
                    }
                })
                .collect()
        });

    let ans = dp.into_iter().max().unwrap().unwrap();

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
