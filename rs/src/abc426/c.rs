fn main() {
    input! {
        n: usize, q: usize,
        xy: [(Usize1, Usize1); q],
    };

    xy.into_iter()
        .scan((vec![1; n], 0), |(nums, i0), (x, y)| {
            if *i0 <= x {
                let s = nums[*i0..=x].iter().sum::<usize>();
                *i0 = x + 1;
                nums[y] += s;

                Some(s)
            } else {
                Some(0)
            }
        })
        .for_each(|ans| {
            println!("{ans}");
        });
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
