fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let mut negs = a
        .iter()
        .copied()
        .filter(|&x| x < 0)
        .sorted()
        .collect::<Vec<_>>();
    let mut poss = a
        .iter()
        .copied()
        .filter(|&x| x > 0)
        .sorted()
        .rev()
        .collect::<Vec<_>>();

    let mut ans = 0;
    let mut cur = 0;
    while !negs.is_empty() || !poss.is_empty() {
        if negs
            .last()
            .copied()
            .map(|x| x.abs_diff(cur))
            .unwrap_or(u64::MAX)
            <= poss
                .last()
                .copied()
                .map(|x| x.abs_diff(cur))
                .unwrap_or(u64::MAX)
        {
            let x = negs.pop().unwrap();
            ans += x.abs_diff(cur);
            cur = x;
        } else {
            let x = poss.pop().unwrap();
            ans += x.abs_diff(cur);
            cur = x;
        }
    }

    println!("{ans}");
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
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
use rand::{Rng, SeedableRng, rngs::SmallRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
