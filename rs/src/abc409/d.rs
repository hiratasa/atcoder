fn main() {
    input! {
        t: usize,
        cases: [(usize, Chars); t],
    };

    cases
        .into_iter()
        .map(|(n, s)| {
            if let Some(i) = (0..n - 1).find(|&i| s[i] > s[i + 1]) {
                let j = (i + 1..n).find(|&j| s[j] > s[i]).unwrap_or(n);

                s[..i]
                    .iter()
                    .chain(&s[i + 1..j])
                    .chain(once(&s[i]))
                    .chain(&s[j..])
                    .copied()
                    .collect::<String>()
            } else {
                s.into_iter().collect::<String>()
            }
        })
        .for_each(|ans| {
            println!("{ans}");
        });
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
