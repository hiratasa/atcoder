fn main() {
    input! {
        x: usize, y: usize,
    };

    let rev = |z: usize| {
        successors(Some(z), |&w| Some(w / 10))
            .take_while(|&w| w > 0)
            .map(|w| w % 10)
            .fold(0, |a, b| 10 * a + b)
    };

    let ans = successors(Some((x, y)), |&(a, b)| Some((b, rev(a + b))))
        .nth(8)
        .unwrap()
        .1;

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
