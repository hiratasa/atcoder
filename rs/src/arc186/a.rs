fn main() {
    input! {
        n: usize, q: usize,
        ks: [usize; q],
    };

    let mut init = vec![vec![BitSet::new(n * n + 1); n + 1]; n + 1];
    for i in 0..=n {
        init[i][0].set(0, true);
        init[0][i].set(0, true);
    }
    let dp = iproduct!(1..=n, 1..=n).fold(init, |mut dp, (i, j)| {
        dp[i][j].set(0, true);
        if i > 1 && j > 1 {
            dp[i][j].set(i * j, true);
        }

        iproduct!(0..i, 0..j).skip(1).for_each(|(k, l)| {
            let k2 = i - k;
            let l2 = j - l;

            for s in 0..=k * l {
                if dp[k][l][s] {
                    let tmp = &dp[k2][l2] << s;
                    dp[i][j] |= &tmp;
                }
            }
        });

        // eprintln!("{i},{j}: {:?}", dp[i][j]);

        dp
    });

    // println!("{dp:?}");

    ks.into_iter().for_each(|k| {
        if dp[n][n][n * n - k] {
            println!("Yes");
        } else {
            println!("No");
        }
    })
}

#[allow(dead_code)]
fn solve0(n: usize, k: usize) -> Vec<Vec<usize>> {
    let n2 = n * n;

    (0usize..1 << n2)
        .map(|s| {
            let grid = (0..n)
                .map(|i| (s >> (i * n)) & ((1 << n) - 1))
                .collect::<Vec<_>>();

            let row_sums = grid
                .iter()
                .map(|row| row.count_ones() as usize)
                .collect::<Vec<_>>();
            let col_sums = (0..n)
                .map(|j| grid.iter().map(|row| (row >> j) & 1).sum::<usize>())
                .collect::<Vec<_>>();

            (grid, row_sums, col_sums)
        })
        .sorted_by_key(|(_, row_sums, col_sums)| (row_sums.clone(), col_sums.clone()))
        .group_by(|(_, row_sums, col_sums)| (row_sums.clone(), col_sums.clone()))
        .into_iter()
        .map(|(_, it)| {
            let grids = it.map(|(grid, _, _)| grid).collect::<Vec<_>>();

            let k = iproduct!(0..n, 0..n)
                .filter(|&(i, j)| grids.iter().map(|grid| (grid[i] >> j) & 1).all_equal())
                .count();
            (grids, k)
        })
        .filter(|(_, kk)| *kk == k)
        .map(|(grids, _)| grids)
        .sorted()
        .next()
        .unwrap()
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
