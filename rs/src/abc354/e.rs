use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    };

    if solve((1 << n) - 1, &ab, &mut HashMap::new()) {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}

fn solve(s: usize, ab: &[(usize, usize)], memo: &mut HashMap<usize, bool>) -> bool {
    if let Some(&r) = memo.get(&s) {
        return r;
    }

    let n = ab.len();
    let win = (0..n)
        .filter(|&i| s & (1 << i) > 0)
        .tuple_combinations()
        .filter(|&(i, j)| ab[i].0 == ab[j].0 || ab[i].1 == ab[j].1)
        .any(|(i, j)| !solve(s ^ (1 << i) ^ (1 << j), ab, memo));

    memo.insert(s, win);

    win
}
