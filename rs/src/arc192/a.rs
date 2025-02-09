fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let ans = match n % 4 {
        0 => true,
        1 | 3 => a.iter().copied().any(|x| x == 1),
        2 => (0..2).all(|r| (0..n).filter(|&i| i % 2 == r).any(|i| a[i] == 1)),
        _ => unreachable!(),
    };

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
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
