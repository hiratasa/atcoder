fn main() {
    input! {
        n: u32,
    }

    let mut ans = vec![vec![false; 3usize.pow(n)]; 3usize.pow(n)];
    calc(n, 0, 0, &mut ans);

    for row in ans {
        println!(
            "{}",
            row.into_iter().map(|c| if c { '#' } else { '.' }).join("")
        );
    }
}

fn calc(n: u32, i0: usize, j0: usize, v: &mut [Vec<bool>]) {
    if n == 0 {
        v[i0][j0] = true;
    } else {
        let l = 3usize.pow(n - 1);

        for i in 0..3 {
            for j in 0..3 {
                if (i, j) == (1, 1) {
                    continue;
                }

                calc(n - 1, i0 + i * l, j0 + j * l, v);
            }
        }
    }
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::*,
    mem::{replace, take},
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
