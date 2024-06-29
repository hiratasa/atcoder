fn main() {
    input! {
        n: usize, k: usize,
        mut p: [usize; n],
    };

    let mut ans = vec![];
    while let Some((i, _)) = (0..n)
        .tuple_combinations()
        .filter(|&(i, j)| j - i >= k && p[i] > p[j])
        .min_by_key(|&(i, j)| (p[i], Reverse(p[j])))
    {
        let js = (i + k..n)
            .filter(|&j| p[i] > p[j])
            .sorted_by_key(|&j| Reverse(p[j]))
            .collect::<Vec<_>>();
        for j in js {
            ans.push((i + 1, j + 1));
            p.swap(i, j);
        }
    }

    println!("{}", ans.len());
    for (i, j) in ans {
        println!("{i} {j}");
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
