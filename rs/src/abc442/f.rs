fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let dp = s.into_iter().fold(vec![0usize; n + 1], |prev, row| {
        let mut b = row.iter().filter(|&&c| c == '#').count();
        let mut w = 0;

        let mut next = vec![0; n + 1];
        let mut mi = usize::MAX;
        for i in (0..=n).rev() {
            if i < n {
                if row[i] == '.' {
                    w += 1;
                } else {
                    b -= 1;
                }
            }
            mi = min(mi, prev[i]);
            next[i] = mi + b + w;
        }

        next
    });

    let ans = dp.iter().min().unwrap();

    println!("{ans}");
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_n, repeat_with, successors},
    mem::{replace, swap, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, iterate, izip};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
