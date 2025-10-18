fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
    };

    let ans = iproduct!(0..h, 0..w)
        .filter(|&(i, j)| s[i][j] == '#')
        .all(|(i, j)| {
            let c = [(0, 1), (0, -1), (1, 0), (-1, 0)]
                .into_iter()
                .filter_map(|(di, dj)| Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?)))
                .filter(|&(ni, nj)| ni < h && nj < w)
                .filter(|&(ni, nj)| s[ni][nj] == '#')
                .count();

            c == 2 || c == 4
        });

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
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
