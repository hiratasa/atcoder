fn main() {
    input! {
        n: usize, q: usize,
        lrc: [(Usize1, usize, usize); q],
    };

    let (c, k) = lrc
        .into_iter()
        .sorted_by_key(|&(_, _, c)| c)
        .scan(
            (0..n).map(|i| (i, i + 1)).collect::<BTreeMap<_, _>>(),
            |intervals, (l, r, c)| {
                let mut cost = c;
                let mut k = 0;
                while let Some((&ll, &rr)) =
                    intervals.range(..=l).next_back().filter(|&(_, &rr)| rr < r)
                {
                    cost += c;
                    k += 1;
                    let rr2 = intervals.remove(&rr).unwrap();
                    intervals.insert(ll, rr2);
                }

                Some((cost, k))
            },
        )
        .fold((0, 0), |(c, k), (c1, k1)| (c + c1, k + k1));

    if k < n - 1 {
        println!("-1");
    } else {
        println!("{c}");
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
