fn main() {
    input! {
        h: usize, w: usize,
        s: (Usize1, Usize1),
        grid: [Chars; h],
        x: Chars,
    };

    let ans = x.into_iter().fold(s, |(i, j), c| {
        let (di, dj) = match c {
            'L' => (0, -1),
            'R' => (0, 1),
            'U' => (-1, 0),
            'D' => (1, 0),
            _ => unreachable!(),
        };

        Some(())
            .and_then(|_| Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?)))
            .filter(|&(i, j)| i < h && j < w)
            .filter(|&(i, j)| grid[i][j] != '#')
            .unwrap_or((i, j))
    });

    println!("{} {}", ans.0 + 1, ans.1 + 1);
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
