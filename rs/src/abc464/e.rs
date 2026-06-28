fn main() {
    input! {
        h: usize, w: usize, q: usize,
        rcx: [(Usize1, Usize1, char); q],
    };

    let mut grid = rcx.into_iter().enumerate().fold(
        vec![vec![(0, 'A'); w + 1]; h + 1],
        |mut grid, (i, (r, c, x))| {
            grid[r][c] = (i + 1, x);
            grid
        },
    );

    for i in (0..h).rev() {
        for j in (0..w).rev() {
            grid[i][j] = max(grid[i][j], max(grid[i + 1][j], grid[i][j + 1]));
        }
    }

    for i in 0..h {
        println!(
            "{}",
            grid[i].iter().take(w).copied().map(|(_, c)| c).join("")
        );
    }
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_n, repeat_with, successors},
    mem::{replace, swap, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, iterate, izip};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
