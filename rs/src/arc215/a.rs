fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize, k: usize, l: usize,
            mut a: [usize; n],
        };

        a.sort();
        let q = a
            .iter()
            .copied()
            .tuple_windows()
            .map(|(x, y)| y - x)
            .sorted()
            .rev()
            .chain(once(0))
            .collect::<Vec<_>>();

        let ans = q
            .iter()
            .enumerate()
            .scan((a[0], a[n - 1], 0), |(x0, x1, s), (i, c)| {
                let r = if k >= i + 1 {
                    *s + max(*x0, l - *x1) + (k - 1 - i) * (*x0 + (l - *x1))
                } else if k == i {
                    *s
                } else {
                    return None;
                };

                *s += c / 2;
                *x0 += c / 2;
                *x1 -= c / 2;

                Some(r)
            })
            .max()
            .unwrap();

        println!("{ans}");
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
