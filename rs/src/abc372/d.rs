fn main() {
    input! {
        n: usize,
        h: [usize; n],
    };

    let (mut s, _) = h.iter().copied().enumerate().fold(
        (vec![0; n], vec![(0usize, usize::MAX)]),
        |(mut s, mut t), (i, x)| {
            while t[t.len() - 1].1 < x {
                t.pop();
            }

            let i0 = t[t.len() - 1].0;
            s[i0.saturating_sub(1)] += 1;
            s[i] -= 1;

            t.push((i + 1, x));

            (s, t)
        },
    );

    for i in 1..n {
        s[i] += s[i - 1];
    }

    println!("{}", s.into_iter().join(" "));
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
