fn main() {
    input! {
        n: usize,
    };

    let a = (0..n)
        .map(|i| {
            input! {
                a: [Usize1; i + 1],
            };
            a
        })
        .collect::<Vec<_>>();

    let ans = (0..n).fold(0, |c, x| a[max(c, x)][min(c, x)]);

    println!("{}", ans + 1);
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
