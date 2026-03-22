fn main() {
    input! {
        n: usize,
        c: [usize; n * (n - 1)/ 2],
    };

    let cost = |i: usize, j: usize| {
        let (i, j) = (min(i, j), max(i, j));
        c[(2 * n - i - 1) * i / 2 + (j - i - 1)]
    };

    if (0..n)
        .tuple_combinations()
        .any(|(a, b, c)| cost(a, c) > cost(a, b) + cost(b, c))
    {
        println!("Yes");
    } else {
        println!("No");
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
