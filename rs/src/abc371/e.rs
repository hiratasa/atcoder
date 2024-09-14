fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let ans = a
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .sorted()
        .group_by(|&(x, _)| x)
        .into_iter()
        .map(|(_, it)| {
            n * (n + 1) / 2
                - once(0)
                    .chain(it.map(|(_, i)| i + 1))
                    .chain(once(n + 1))
                    .tuple_windows()
                    .map(|(i, j)| j - i)
                    .map(|l| l * (l - 1) / 2)
                    .sum::<usize>()
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
