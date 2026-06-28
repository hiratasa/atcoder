fn main() {
    input! {
        n: usize, m: usize,
        adb: [(Usize1, Usize1, Usize1); n],
    };

    let events = adb
        .iter()
        .copied()
        .sorted_by_key(|&(_, d, _)| d)
        .collect::<Vec<_>>();

    (0..m)
        .scan(
            (
                adb.iter()
                    .copied()
                    .fold((vec![0; n], 0), |(mut freq, k), (a, _, _)| {
                        freq[a] += 1;

                        if freq[a] == 1 {
                            (freq, k + 1)
                        } else {
                            (freq, k)
                        }
                    }),
                0,
            ),
            |((freq, k), i), d| {
                while *i < events.len() && events[*i].1 == d {
                    let (a, _, b) = events[*i];
                    freq[a] -= 1;
                    if freq[a] == 0 {
                        *k -= 1;
                    }
                    if freq[b] == 0 {
                        *k += 1;
                    }
                    freq[b] += 1;
                    *i += 1;
                }

                Some(*k)
            },
        )
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
