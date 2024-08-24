fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize, m: usize,
            ab: [(Usize1, Usize1); m],
        };

        if solve(n, m, &ab) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn solve(n: usize, m: usize, ab: &[(usize, usize)]) -> bool {
    if n <= 2 {
        return true;
    }

    let k = (n + 1) / 2;
    if m < k * (k - 1) / 2 {
        return false;
    }

    let set = ab
        .iter()
        .copied()
        .flat_map(|(a, b)| [(a, b), (b, a)])
        .collect::<FxHashSet<_>>();

    let adjs = (0..n)
        .tuple_combinations()
        .filter(|&(i, j)| !set.contains(&(i, j)))
        .fold(vec![vec![]; n], |mut adjs, (i, j)| {
            adjs[i].push(j);
            adjs[j].push(i);
            adjs
        });

    let mut colors = vec![None; n];

    for i in 0..n {
        if colors[i].is_none() {
            if !dfs(&adjs, i, false, &mut colors) {
                return false;
            }
        }
    }

    true
}

fn dfs(adjs: &[Vec<usize>], v: usize, color: bool, colors: &mut [Option<bool>]) -> bool {
    if let Some(c) = colors[v] {
        return c == color;
    }

    colors[v] = Some(color);

    for &u in &adjs[v] {
        if !dfs(adjs, u, !color, colors) {
            return false;
        }
    }

    true
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
