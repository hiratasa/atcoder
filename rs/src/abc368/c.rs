fn main() {
    input! {
        n: usize,
        h: [usize; n],
    };

    let ans = h.into_iter().fold(0, |t, x| {
        let (t, x) = if t % 3 == 1 { (t + 1, x - 1) } else { (t, x) };

        let (t, x) = if t % 3 == 2 && x > 0 {
            (t + 1, x.saturating_sub(3))
        } else {
            (t, x)
        };

        let (t, x) = (t + x / 5 * 3, x % 5);

        match x {
            0 => t,
            1 => t + 1,
            2 => t + 2,
            3 | 4 => t + 3,
            _ => unreachable!(),
        }
    });

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
