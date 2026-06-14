fn main() {
    input! {
        t: usize,
        cases: [(i64, i64, i64, i64); t],
    };

    cases
        .into_iter()
        .map(|(a, b, x, y)| {
            let x = x.abs();
            let y = y.abs();

            let m = x + y;

            let c = (x + y + 1) / 2;
            let d = (x + y) / 2;

            let a = min(a, 3 * b);
            let b = min(3 * a, b);

            let (a, b, x, y) = if a <= b { (a, b, x, y) } else { (b, a, y, x) };

            if x <= c {
                a * (x + d) + b * (y - d)
            } else {
                a * (c + y) + b * (x - c)
            }
        })
        .for_each(|ans| {
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
