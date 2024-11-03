fn main() {
    input! {
        h: usize, w: usize, k: usize,
        s: [Chars; h],
    };

    println!(
        "{}",
        iproduct!(0..h, 0..w)
            .map(|(i, j)| calc(i, j, k, &s, &mut vec![vec![false; w]; h]))
            .sum::<usize>()
    );
}

fn calc(i: usize, j: usize, k: usize, grid: &[Vec<char>], visited: &mut [Vec<bool>]) -> usize {
    if visited[i][j] {
        return 0;
    }
    if grid[i][j] == '#' {
        return 0;
    }
    if k == 0 {
        return 1;
    }

    visited[i][j] = true;

    let h = grid.len();
    let w = grid[0].len();
    let r = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|(di, dj)| Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?)))
        .filter(|&(ni, nj)| ni < h && nj < w)
        .map(|(ni, nj)| calc(ni, nj, k - 1, grid, visited))
        .sum::<usize>();

    visited[i][j] = false;

    r
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
