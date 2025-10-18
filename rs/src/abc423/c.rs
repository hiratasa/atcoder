fn main() {
    input! {
        n: usize, r: usize,
        l: [usize; n],
    };

    let i0 = l.iter().take_while(|&&c| c == 1).count();
    let i1 = n - l.iter().rev().take_while(|&&c| c == 1).count();

    if i0 == n {
        println!("0");
        return;
    }

    let i0 = min(i0, r);
    let i1 = max(i1, r);

    let ans = l[i0..i1].iter().map(|&x| 1 + x).sum::<usize>();

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
