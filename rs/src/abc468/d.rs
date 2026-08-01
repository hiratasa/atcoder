fn main() {
    input! {
        s: Chars,
    };

    let n = s.len();

    let ans0 = (0..n)
        .map(|i| {
            (0..n)
                .scan(0, |c, j| {
                    if j > i {
                        None
                    } else if i + j >= n {
                        None
                    } else if s[i - j] == s[i + j] {
                        Some(())
                    } else if *c == 0 {
                        *c = 1;
                        Some(())
                    } else {
                        None
                    }
                })
                .count()
        })
        .sum::<usize>();

    let ans1 = (0..n)
        .map(|i| {
            (0..n)
                .scan(0, |c, j| {
                    if j > i {
                        None
                    } else if i + 1 + j >= n {
                        None
                    } else if s[i - j] == s[i + 1 + j] {
                        Some(())
                    } else if *c == 0 {
                        *c = 1;
                        Some(())
                    } else {
                        None
                    }
                })
                .count()
        })
        .sum::<usize>();

    let ans = ans0 + ans1;

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
