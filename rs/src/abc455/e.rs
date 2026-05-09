fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let t = s
        .into_iter()
        .scan((0i64, 0i64, 0i64), |(na, nb, nc), ch| {
            if ch == 'A' {
                *na += 1;
            } else if ch == 'B' {
                *nb += 1;
            } else {
                *nc += 1;
            }

            Some((*na, *nb, *nc))
        })
        .collect::<Vec<_>>();
    let ans = n * (n + 1) / 2
        + 2 * t
            .iter()
            .copied()
            .map(|(na, nb, nc)| (nb - na, nc - nb))
            .chain(once((0, 0)))
            .counts()
            .values()
            .map(|x| x * (x - 1) / 2)
            .sum::<usize>()
        - t.iter()
            .copied()
            .map(|(na, nb, _)| na - nb)
            .chain(once(0))
            .counts()
            .values()
            .map(|x| x * (x - 1) / 2)
            .sum::<usize>()
        - t.iter()
            .copied()
            .map(|(_, nb, nc)| nc - nb)
            .chain(once(0))
            .counts()
            .values()
            .map(|x| x * (x - 1) / 2)
            .sum::<usize>()
        - t.iter()
            .copied()
            .map(|(na, _, nc)| nc - na)
            .chain(once(0))
            .counts()
            .values()
            .map(|x| x * (x - 1) / 2)
            .sum::<usize>();

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
