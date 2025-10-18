fn main() {
    input! {
        t: usize,
        cases: [[(usize, usize, usize)]; t],
    };

    cases
        .into_iter()
        .map(|abc| {
            let (sum, a0, a1) = abc.into_iter().fold((0, 0, 0), |(sum, a0, a1), (a, b, c)| {
                if a + c <= b {
                    (sum + a + c, a0 + a, a1 + a)
                } else {
                    (
                        sum + b,
                        a0 + b.saturating_sub(c),
                        a1 + (b - b.saturating_sub(a)),
                    )
                }
            });

            [a0, a1, sum / 2, (sum + 1) / 2]
                .into_iter()
                .filter(|&x| a0 <= x && x <= a1)
                .map(|x| min(x, sum - x))
                .max()
                .unwrap()
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
