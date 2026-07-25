fn main() {
    input! {
        n: usize,
        x: [[usize; n]; n],
    };

    let y = x
        .iter()
        .map(|row| {
            row.iter()
                .copied()
                .map(|x| [2, 4, 1, 3][x - 1])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for row in y {
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
