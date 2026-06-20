fn main() {
    input! {
        t: usize,
        cases: [(i64, i64, i64); t],
    };

    cases
        .into_iter()
        .map(|(a, b, c)| {
            let n = a + b + c;
            if n % 2 == 0 {
                if (a == b && c == 0) || (b == c && a == 0) || (c == a && b == 0) {
                    return n / 2;
                }
            }

            let ans0 = (0..3)
                .map(|i| {
                    let [a, b, c] = if i == 0 {
                        [a, b, c]
                    } else if i == 1 {
                        [b, c, a]
                    } else {
                        [c, a, b]
                    };

                    // x <= a
                    // y + x + 1 <= b
                    // y + 1 <= c
                    // => x + y <= min(a + c - 1, b - 1)
                    let ans0 = min(a + c - 1, b - 1).max(0);

                    // x <= a
                    // x + 1 <= b
                    let ans1 = min(a, b - 1).max(0);

                    max(ans0, ans1)
                })
                .max()
                .unwrap();

            // x + z + 1 <= a
            // y + x + 1 <= b
            // z + y + 1 <= c
            // => 2 * (x + y + z) + 3 <= a + b + c

            // y + z <= a + b - 2 - 2*x
            // y + z <= c - 1
            // => y + z <= min(a + b - 2 - 2*x, c - 1)
            // => x + y + z <= x + min(a + b - 2 - 2*x, c - 1)

            let limit = min(a - 1, b - 1).max(0);
            let x0 = ((a + b - c - 1).max(0) / 2).min(limit);
            let x1 = (x0 + 1).min(limit);

            let ans1 = max(
                x0 + min(a + b - 2 - 2 * x0, c - 1),
                x1 + min(a + b - 2 - 2 * x1, c - 1),
            );

            max(ans0, ans1)
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
