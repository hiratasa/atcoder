fn main() {
    input! {
        n: usize, m: usize,
        s: [Chars; n],
    };

    let s = s
        .into_iter()
        .map(|s| {
            s.into_iter()
                .map(|x| (x == 'o') as u32)
                .fold(0, |x, y| (x << 1) + y)
        })
        .collect::<Vec<_>>();

    let ans = (1u32..1 << n)
        .filter(|&t| {
            (0..n)
                .filter(|&i| t & (1 << i) > 0)
                .map(|i| s[i])
                .fold(0, |x, y| x | y)
                == (1 << m) - 1
        })
        .map(|t| t.count_ones())
        .min()
        .unwrap();

    println!("{ans}");
}

#[allow(unused_imports)]
use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::*,
    mem::{replace, take},
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
