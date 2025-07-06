fn main() {
    input! {
        h: usize, w: usize, k: usize,
        rc: [(Usize1, Usize1); k],
    };

    let mut ans = vec![vec![usize::MAX; w]; h];
    let mut nums = vec![vec![0; w]; h];
    let mut q = VecDeque::new();
    for (r, c) in rc {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(|(dr, dc)| Some((r.checked_add_signed(dr)?, c.checked_add_signed(dc)?)))
            .filter(|&(nr, nc)| nr < h && nc < w)
            .for_each(|(nr, nc)| {
                q.push_back((nr, nc, 1usize));
            });
        ans[r][c] = 0;
        nums[r][c] = 4;
    }

    while let Some((r, c, d)) = q.pop_front() {
        nums[r][c] += 1;

        if nums[r][c] == 2 {
            ans[r][c] = d;
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|(dr, dc)| Some((r.checked_add_signed(dr)?, c.checked_add_signed(dc)?)))
                .filter(|&(nr, nc)| nr < h && nc < w)
                .for_each(|(nr, nc)| {
                    q.push_back((nr, nc, d + 1));
                });
        }
    }

    println!(
        "{}",
        ans.iter()
            .flatten()
            .filter(|&&x| x != usize::MAX)
            .sum::<usize>()
    );
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
