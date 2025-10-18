fn main() {
    input! {
        n: usize, q: usize,
        a: [i64; n],
        lr: [(Usize1, usize); q],
    };

    let sums0 = once(0)
        .chain(a.iter().copied())
        .cumsum::<i64>()
        .collect::<Vec<_>>();
    let sums1 = once(0)
        .chain(a.iter().copied().enumerate().map(|(i, x)| (i as i64) * x))
        .cumsum::<i64>()
        .collect::<Vec<_>>();
    let sums2 = once(0)
        .chain(
            a.iter()
                .copied()
                .enumerate()
                .map(|(i, x)| (i as i64) * (i as i64) * x),
        )
        .cumsum::<i64>()
        .collect::<Vec<_>>();

    lr.into_iter()
        .map(|(l, r)| {
            let s0 = sums0[r] - sums0[l];
            let s1 = sums1[r] - sums1[l];
            let s2 = sums2[r] - sums2[l];

            let l = l as i64;
            let r = r as i64;

            // Σ (i - l + 1) * (r - i) * a[i]
            // = -s2 + (l + r - 1) * s1 - (l - 1) * r * s0
            (l + r - 1) * s1 - (l - 1) * r * s0 - s2
        })
        .for_each(|ans| println!("{ans}"));
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
