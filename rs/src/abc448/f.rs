fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    };

    let mx = xy.iter().copied().map(|t| t.0).max().unwrap() + 1;
    let my = xy.iter().copied().map(|t| t.1).max().unwrap() + 1;

    let b = ((mx as f64) * (my as f64) / (n as f64)).sqrt().floor() as usize;

    let ans0 = (0..n)
        .sorted_by_key(|&i| (xy[i].0 / b, xy[i].1))
        .chunk_by(|&i| xy[i].0 / b)
        .into_iter()
        .enumerate()
        .flat_map(|(i, (_, it))| {
            let mut idxs = it.collect::<Vec<_>>();

            if i % 2 > 0 {
                idxs.reverse();
            }

            idxs
        })
        .collect::<Vec<_>>();
    let ans = ans0
        .into_iter()
        .cycle()
        .skip_while(|&i| i != 0)
        .take(n)
        .collect::<Vec<_>>();

    println!("{}", ans.iter().map(|&i| i + 1).join(" "));
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
