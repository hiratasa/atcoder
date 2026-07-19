fn main() {
    input! {
        n: usize, m: usize,
        rc: [(Usize1, Usize1); m],
    };

    let (rows, cols) = rc.iter().copied().fold(
        (vec![None; n], vec![None; n]),
        |(mut rows, mut cols), (r, c)| {
            if let Some(c2) = rows[r] {
                rows[r] = None;
                cols[c2] = None;
            }
            if let Some(r2) = cols[c] {
                cols[c] = None;
                rows[r2] = None;
            }

            rows[r] = Some(c);
            cols[c] = Some(r);

            (rows, cols)
        },
    );

    let ans = rows.iter().copied().flatten().count();

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
