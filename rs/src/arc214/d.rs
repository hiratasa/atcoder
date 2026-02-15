fn main() {
    input! {
        n: usize,
    };

    let mut ans = vec![vec![0; n]; n];
    for s in 1..2 * (n - 1) {
        (0..=s)
            .map(|i| (i, s - i))
            .filter(|&(i, j)| i < n && j < n)
            .step_by(2)
            .for_each(|(i, j)| {
                ans[i][j] = 1usize << (s - 1);
            });
    }

    for row in ans {
        println!("{}", row.iter().join(" "));
    }
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
