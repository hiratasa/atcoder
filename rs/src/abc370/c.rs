fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let n = s.len();

    println!(
        "{}",
        izip!(s.iter(), t.iter()).filter(|&(c, d)| c != d).count()
    );

    println!(
        "{}",
        (0..n)
            .filter(|&i| s[i] > t[i])
            .chain((0..n).rev().filter(|&i| s[i] < t[i]))
            .scan(s.clone(), |u, i| {
                u[i] = t[i];
                Some(u.iter().collect::<String>())
            })
            .join("\n")
    );
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
