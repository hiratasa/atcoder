fn main() {
    input! {
        n: usize, q: usize,
        s: [Chars; n],
        rects: [(Usize1, Usize1, Usize1, Usize1); q],
    };

    let mut t = iproduct!(0..n - 1, 0..n - 1).fold(vec![vec![0; n + 1]; n + 1], |mut t, (i, j)| {
        if s[i][j] == '.' && s[i][j + 1] == '.' && s[i + 1][j] == '.' && s[i + 1][j + 1] == '.' {
            t[i + 1][j + 1] = 1;
        }

        t
    });

    // eprintln!("{t:?}");

    for i in 0..=n {
        for j in 0..n {
            t[i][j + 1] += t[i][j];
        }
    }

    for i in 0..n {
        for j in 0..=n {
            t[i + 1][j] += t[i][j];
        }
    }

    // eprintln!("{t:?}");

    rects
        .into_iter()
        .map(|(u, d, l, r)| t[d][r] + t[u][l] - t[d][l] - t[u][r])
        .for_each(|ans| {
            println!("{ans}");
        })
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
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
