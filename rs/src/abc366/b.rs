fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let m = s.iter().map(|x| x.len()).max().unwrap();

    let ans = (0..m)
        .map(|i| {
            (0..n)
                .filter_map(|j| s[n - 1 - j].get(i).copied().map(|x| (j, x)))
                .scan(0, |l, (j, x)| {
                    let k = j - *l;
                    *l = j + 1;

                    Some((k, x))
                })
                .flat_map(|(k, x)| repeat_n('*', k).chain(once(x)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for row in ans {
        println!("{}", row.iter().join(""));
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
