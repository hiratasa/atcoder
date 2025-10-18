fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    };

    let mut init = vec![];
    let adjs = ab
        .iter()
        .enumerate()
        .fold(vec![vec![]; n], |mut adjs, (i, &(a, b))| {
            if a > 0 {
                adjs[a - 1].push(i);
                adjs[b - 1].push(i);
            } else {
                init.push(i);
            }
            adjs
        });

    let mut visited = vec![false; n];
    for v in init {
        dfs(&adjs, &mut visited, v);
    }

    println!("{}", visited.iter().filter(|&&v| v).count());
}

fn dfs(adjs: &[Vec<usize>], visited: &mut [bool], v: usize) {
    if visited[v] {
        return;
    }
    visited[v] = true;
    for &u in &adjs[v] {
        dfs(adjs, visited, u);
    }
}

use std::vec;
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
