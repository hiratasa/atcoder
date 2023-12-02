use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::{input, marker::Usize1};
use rustc_hash::FxHashSet;

fn main() {
    input! {
        n: usize, m: usize, l: usize,
        a: [usize; n],
        b: [usize; m],
        cd: [(Usize1, Usize1); l],
    };

    let banned = cd.iter().copied().collect::<FxHashSet<_>>();

    let sorted_a_idxs = (0..n).sorted_by_key(|&i| a[i]).collect::<Vec<_>>();
    let sorted_b_idxs = (0..m).sorted_by_key(|&i| b[i]).collect::<Vec<_>>();

    let mut q = BinaryHeap::new();
    for i in 0..n {
        let idx0 = sorted_a_idxs[i];
        let idx1 = sorted_b_idxs[m - 1];
        q.push((a[idx0] + b[idx1], i, m - 1));
    }

    while let Some((price, i, j)) = q.pop() {
        let idx0 = sorted_a_idxs[i];
        let idx1 = sorted_b_idxs[j];
        if !banned.contains(&(idx0, idx1)) {
            println!("{price}");
            return;
        }

        if j > 0 {
            let idx1_next = sorted_b_idxs[j - 1];
            q.push((a[idx0] + b[idx1_next], i, j - 1));
        }
    }
}
