fn main() {
    input! {
        t: usize,
        cases: [usize; t],
    };

    cases
        .into_iter()
        .map(|n| {
            once(1)
                .chain(
                    (2..=n + 1)
                        .tuples()
                        .enumerate()
                        .flat_map(|(i, (x, y))| if i % 2 == 0 { [y, x] } else { [x, y] })
                        .filter(|&x| x <= n),
                )
                .collect::<Vec<_>>()
        })
        .for_each(|ans| {
            // let x = ans
            //     .iter()
            //     .copied()
            //     .scan(0, |x, y| {
            //         *x ^= y;
            //         Some(*x)
            //     })
            //     .sum::<usize>();
            // eprintln!("#{x}");
            println!("{}", ans.iter().join(" "));
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
