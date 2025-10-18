fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t: [Chars; n],
    };

    let ans = (0..4)
        .scan(s, |s, _| {
            let ss = s.clone();

            *s = (0..n)
                .map(|i| (0..n).map(|j| s[n - 1 - j][i]).collect())
                .collect();

            Some(ss)
        })
        .enumerate()
        .map(|(idx, s)| {
            idx + iproduct!(0..n, 0..n)
                .filter(|&(i, j)| s[i][j] != t[i][j])
                .count()
        })
        .min()
        .unwrap();

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
