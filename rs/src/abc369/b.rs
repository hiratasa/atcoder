fn main() {
    input! {
        n: usize,
        a: [(usize, char); n],
    };

    let ans0 = a
        .iter()
        .copied()
        .filter(|&(_, x)| x == 'L')
        .tuple_windows()
        .map(|((i, _), (j, _))| i.abs_diff(j))
        .sum::<usize>();
    let ans1 = a
        .iter()
        .copied()
        .filter(|&(_, x)| x == 'R')
        .tuple_windows()
        .map(|((i, _), (j, _))| i.abs_diff(j))
        .sum::<usize>();

    println!("{}", ans0 + ans1);
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
