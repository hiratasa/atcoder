fn main() {
    input! {
        n: i64,
        a: [i64; n],
    };

    if let Some(ans) = (1..=n)
        .permutations(n as usize)
        .find(|p| (0..(n as usize)).all(|i| p[i] == a[i] || a[i] == -1))
    {
        println!("Yes");
        println!("{}", ans.iter().join(" "));
    } else {
        println!("No");
    }
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
