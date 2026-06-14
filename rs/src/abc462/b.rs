fn main() {
    input! {
        n: usize,
        a: [[Usize1]; n],
    };

    let ans = a
        .into_iter()
        .enumerate()
        .fold(vec![vec![]; n], |mut ans, (i, v)| {
            for j in v {
                ans[j].push(i);
            }
            ans
        });

    for row in ans {
        println!(
            "{} {}",
            row.len(),
            row.iter().copied().map(|x| x + 1).join(" ")
        );
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
