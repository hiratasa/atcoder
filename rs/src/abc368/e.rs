fn main() {
    input! {
        n: usize, m: usize, x1: usize,
        abst: [(Usize1, Usize1, usize, usize); m],
    };

    let mut init = vec![0; m];
    init[0] = x1;

    let (x, _) = abst
        .into_iter()
        .enumerate()
        .flat_map(|(i, (a, b, s, t))| [(s, true, a, i), (t, false, b, i)])
        .sorted()
        .skip_while(|&(_, departure, _, idx)| !(!departure && idx == 0))
        .fold(
            (init, vec![0usize; n]),
            |(mut x, mut max_departure), (time, departure, v, idx)| {
                if departure {
                    x[idx] = max_departure[v].saturating_sub(time);
                } else {
                    max_departure[v] = max(max_departure[v], time + x[idx]);
                }

                (x, max_departure)
            },
        );

    println!("{}", x[1..].iter().join(" "));
}

#[allow(unused_imports)]
use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_with, successors},
    mem::{replace, swap, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
