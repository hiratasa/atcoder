fn main() {
    input! {
        m: usize,
    };

    let ans = unfold(m, |m| {
        let t = (0..).take_while(|&t| 3usize.pow(t) <= *m).last()?;

        *m -= 3usize.pow(t);

        Some(t)
    })
    .collect::<Vec<_>>();

    println!("{}", ans.len());
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
