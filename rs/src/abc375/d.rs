fn main() {
    input! {
        s: Chars,
    };

    let n = s.len();

    let from_left = once([0; 26])
        .chain(s.iter().scan([0; 26], |nums, &c| {
            nums[c as usize - 'A' as usize] += 1;
            Some(*nums)
        }))
        .collect::<Vec<_>>();
    let from_rights = once([0; 26])
        .chain(s.iter().rev().scan([0; 26], |nums, &c| {
            nums[c as usize - 'A' as usize] += 1;
            Some(*nums)
        }))
        .collect::<Vec<_>>();

    println!(
        "{}",
        (1..n - 1)
            .map(|i| {
                (0..26)
                    .map(|c| from_left[i][c] * from_rights[n - 1 - i][c])
                    .sum::<usize>()
            })
            .sum::<usize>()
    );
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
