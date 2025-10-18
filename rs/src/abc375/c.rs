fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    };

    println!(
        "{}",
        (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| {
                        let d = min(min(i, n - 1 - i), min(j, n - 1 - j));

                        match d % 4 {
                            0 => a[n - 1 - j][i],
                            1 => a[n - 1 - i][n - 1 - j],
                            2 => a[j][n - 1 - i],
                            3 => a[i][j],
                            _ => unreachable!(),
                        }
                    })
                    .join("")
            })
            .join("\n")
    );
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
