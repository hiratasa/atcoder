fn main() {
    input! {
        h: usize, w: usize,
        mut s: [Chars; h],
    };

    let mut t = iproduct!(0..h, 0..w)
        .filter(|&(i, j)| {
            s[i][j] == '.'
                && [(0, 1), (1, 0), (0, -1), (-1, 0)]
                    .into_iter()
                    .filter_map(|(di, dj)| {
                        Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?))
                    })
                    .filter(|&(ni, nj)| ni < h && nj < w && s[ni][nj] == '#')
                    .count()
                    == 1
        })
        .collect::<Vec<_>>();

    while !t.is_empty() {
        for &(i, j) in &t {
            s[i][j] = '#';
        }

        t = t
            .into_iter()
            .flat_map(|(i, j)| {
                [(0, 1), (1, 0), (0, -1), (-1, 0)]
                    .into_iter()
                    .filter_map(move |(di, dj)| {
                        Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?))
                    })
                    .filter(|&(ni, nj)| ni < h && nj < w)
            })
            .filter(|&(i, j)| {
                s[i][j] == '.'
                    && [(0, 1), (1, 0), (0, -1), (-1, 0)]
                        .into_iter()
                        .filter_map(|(di, dj)| {
                            Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?))
                        })
                        .filter(|&(ni, nj)| ni < h && nj < w && s[ni][nj] == '#')
                        .count()
                        == 1
            })
            .collect::<Vec<_>>();
    }

    let ans = iproduct!(0..h, 0..w)
        .filter(|&(i, j)| s[i][j] == '#')
        .count();

    println!("{ans}");
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
