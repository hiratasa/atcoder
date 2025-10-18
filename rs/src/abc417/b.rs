fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
        b: [usize; m],
    };

    let ans = b.into_iter().fold(a, |mut a, x| {
        if let Some(pos) = a.iter().position(|&y| y == x) {
            a.remove(pos);
        }
        a
    });
    println!("{}", ans.iter().join(" "));
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
