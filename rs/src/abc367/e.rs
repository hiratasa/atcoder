fn main() {
    input! {
        n: usize, k: usize,
        x: [Usize1; n],
        a: [usize; n],
    };

    let mut y = vec![vec![0; n]; 60];
    y[0] = x;
    for i in 0..59 {
        for j in 0..n {
            y[i + 1][j] = y[i][y[i][j]];
        }
    }

    let mut ans = (0..n)
        .map(|mut i| {
            let mut k = k;
            for j in (0..60).rev() {
                if k >= 1 << j {
                    i = y[j][i];
                    k -= 1 << j;
                }
            }

            i
        })
        .map(|i| a[i]);

    println!("{}", ans.join(" "));
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
