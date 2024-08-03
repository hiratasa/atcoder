fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let ans = (0..30)
        .map(|i| {
            a.iter()
                .copied()
                .map(|x| (x >> i) & 1)
                .scan([0usize; 2], |s, x| {
                    s[0] += 1;
                    if x == 0 {
                        Some(s[1])
                    } else {
                        s.swap(0, 1);
                        Some(s[1] - 1)
                    }
                })
                .sum::<usize>()
                * (1 << i)
        })
        .sum::<usize>();

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
