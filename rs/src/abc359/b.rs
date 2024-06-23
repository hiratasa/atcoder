fn main() {
    input! {
        n: usize,
        a: [usize; 2 * n],
    };

    println!(
        "{}",
        (1..=n)
            .filter(|&i| a.iter().position(|&x| x == i).unwrap() + 2
                == a.iter().rposition(|&x| x == i).unwrap())
            .count()
    );
}

#[allow(unused_imports)]
use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::*,
    mem::{replace, take},
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
