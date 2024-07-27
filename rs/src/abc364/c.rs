fn main() {
    input! {
        n: usize, x: usize, y: usize,
        a: [usize; n],
        b: [usize; n],
    };

    let ans0 = a
        .into_iter()
        .sorted()
        .rev()
        .cumsum::<usize>()
        .position(|z| z > x)
        .map_or(n, |m| m + 1);
    let ans1 = b
        .into_iter()
        .sorted()
        .rev()
        .cumsum::<usize>()
        .position(|z| z > y)
        .map_or(n, |m| m + 1);

    let ans = min(ans0, ans1);

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
