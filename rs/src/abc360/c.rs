fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        w: [usize; n],
    };

    let sum = w.iter().copied().sum::<usize>();

    let maxs = izip!(a, w).fold(vec![0; n], |mut maxs, (i, x)| {
        maxs[i] = max(maxs[i], x);
        maxs
    });

    let ans = sum - maxs.iter().copied().sum::<usize>();

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
