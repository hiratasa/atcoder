fn main() {
    input! {
        n: usize,
        hl: [(usize, usize); n],
        q: usize,
        t: [usize; q],
    };

    t.iter()
        .copied()
        .enumerate()
        .sorted_by_key(|&(i, t)| t)
        .scan(
            hl.iter().copied().collect::<BinaryHeap<_>>(),
            |q, (i, t)| {
                while let Some(&(h, l)) = q.peek()
                    && l <= t
                {
                    q.pop();
                }

                Some((i, q.peek().unwrap().0))
            },
        )
        .sorted()
        .for_each(|(_, ans)| {
            println!("{ans}");
        });
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
