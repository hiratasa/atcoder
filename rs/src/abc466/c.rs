fn main() {
    input_interactive! {
        n: usize,
    };

    let mut j = 2;
    let mut ans = 0;
    for i in 1..=n {
        if i == j {
            j += 1;
        }

        while j <= n && query(i, j) {
            j += 1;
        }

        ans += j - i - 1;
    }

    println!("! {ans}");
}

fn query(i: usize, j: usize) -> bool {
    println!("? {i} {j}");

    input_interactive! {
        s:String,
    };

    s == "Yes"
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
