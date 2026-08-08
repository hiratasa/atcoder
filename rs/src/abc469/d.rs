fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m],
    };

    let (a0, b0) = ab[0];

    let mut ans = 0;

    for x in [a0, b0] {
        let v = ab[1..]
            .iter()
            .copied()
            .filter(|&(a, b)| a != x && b != x)
            .collect::<Vec<_>>();

        if v.is_empty() {
            ans += n - 2;
        } else {
            let (c, d) = v[0];
            ans += [c, d]
                .into_iter()
                .filter(|&y| y != a0 + b0 - x)
                .filter(|&y| v.iter().copied().all(|(a, b)| a == y || b == y))
                .count();
        }
    }

    if ab
        .iter()
        .copied()
        .all(|(a, b)| a == a0 || a == b0 || b == a0 || b == b0)
    {
        ans += 1;
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
