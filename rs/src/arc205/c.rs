fn main() {
    input! {
        n: usize,
        st: [(i64, i64); n],
    };

    let sorted = (0..n).sorted_by_key(|&i| st[i].0).collect::<Vec<_>>();
    // let idxs = sorted
    //     .iter()
    //     .enumerate()
    //     .fold(vec![0; n], |mut idxs, (i, &j)| {
    //         idxs[j] = i;
    //         idxs
    //     });

    let st = sorted.iter().map(|&i| st[i]).collect::<Vec<_>>();

    let mut in_degs = vec![0; n];
    let mut out_edges = vec![vec![]; n];
    for i in 0..n - 1 {
        let gap = st[i + 1].0 - st[i].0;

        let e0 = st[i].1 - st[i].0;
        let e1 = st[i + 1].1 - st[i + 1].0;

        if gap - e0 + e1 <= 0 {
            println!("No");
            return;
        }

        if gap - e0 > 0 && gap + e1 <= 0 {
            in_degs[i + 1] += 1;
            out_edges[i].push(i + 1);
        } else if gap + e1 > 0 && gap - e0 <= 0 {
            in_degs[i] += 1;
            out_edges[i + 1].push(i);
        }
    }

    let mut zeros = (0..n).filter(|&i| in_degs[i] == 0).collect::<Vec<_>>();
    let mut p = vec![];
    while let Some(v) = zeros.pop() {
        p.push(v);
        for &u in &out_edges[v] {
            in_degs[u] -= 1;
            if in_degs[u] == 0 {
                zeros.push(u);
            }
        }
    }

    if p.len() == n {
        println!("Yes");
        println!("{}", p.iter().map(|&i| sorted[i] + 1).join(" "));
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
