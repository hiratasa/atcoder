fn main() {
    input! {
        n: usize, q: usize,
        a: [Usize1; q],
    };

    a.into_iter()
        .scan((0, vec![false; n]), |(s, t), x| {
            *s = match (
                x.checked_sub(1).map(|i| t[i]).unwrap_or(false),
                t[x],
                t.get(x + 1).copied().unwrap_or(false),
            ) {
                (false, false, false) => *s + 1,
                (true, true, true) => *s + 1,
                (true, false, true) => *s - 1,
                (false, true, false) => *s - 1,
                (false, _, true) | (true, _, false) => *s,
            };

            t[x] = !t[x];

            Some(*s)
        })
        .for_each(|ans| {
            println!("{ans}");
        })
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
