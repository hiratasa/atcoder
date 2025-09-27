fn main() {
    input! {
        n: usize,
        p: [usize; n],
        c: [usize; n],
    };

    let colors = izip!(p, c).fold(vec![vec![]; n + 1], |mut colors, (p, c)| {
        colors[c].push(p);
        colors
    });

    let ans = colors
        .into_iter()
        .enumerate()
        .map(|(c, ps)| {
            let len = ps.len();
            let t = ps.into_iter().fold(vec![], |mut t, p| {
                let i = t.binary_search(&p).unwrap_err();

                if i == t.len() {
                    t.push(p);
                } else {
                    t[i] = p;
                }

                t
            });

            c * (len - t.len())
        })
        .sum::<usize>();

    println!("{ans}");
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
