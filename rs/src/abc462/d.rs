fn main() {
    input! {
        n: usize, d: usize,
        st: [(usize, usize); n],
    };

    let mut table = st
        .into_iter()
        .fold(vec![0i64; 1000001], |mut table, (s, t)| {
            if s + d <= t {
                table[s] += 1;
                table[t - d + 1] += -1;
            }
            table
        });

    let mut ans = 0;
    for i in 1..=1000000 {
        table[i] += table[i - 1];
        ans += table[i] * table[i].saturating_sub(1) / 2;
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
