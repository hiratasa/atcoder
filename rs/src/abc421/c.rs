fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let ans0 = s
        .iter()
        .scan((0usize, 0usize), |(na, nb), &c| {
            if c == 'A' {
                *na += 1;

                Some(nb.saturating_sub(*na - 1))
            } else {
                *nb += 1;
                Some(na.saturating_sub(*nb))
            }
        })
        .sum::<usize>();

    let ans1 = s
        .iter()
        .scan((0usize, 0usize), |(na, nb), &c| {
            if c == 'B' {
                *na += 1;

                Some(nb.saturating_sub(*na - 1))
            } else {
                *nb += 1;
                Some(na.saturating_sub(*nb))
            }
        })
        .sum::<usize>();

    let ans = min(ans0, ans1);

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
