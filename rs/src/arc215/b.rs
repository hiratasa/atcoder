fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            a: [Usize1; 2 * n],
        };

        let ans = a
            .iter()
            .copied()
            .scan(
                (vec![None; n], false),
                |(t, prev): &mut (Vec<Option<bool>>, bool), x| {
                    if let Some(b) = t[x] {
                        *prev = !b;
                        Some(*prev)
                    } else {
                        t[x] = Some(*prev);
                        Some(*prev)
                    }
                },
            )
            .tuple_windows()
            .positions(|(x, y)| x != y)
            .map(|i| i + 1)
            .collect::<Vec<_>>();

        println!("{}", ans.len());
        println!("{}", ans.iter().join(" "));
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
