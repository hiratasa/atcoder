fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    };

    let s = s
        .into_iter()
        .rev()
        .fold(0usize, |s, x| (s << 1) | (x == 'B') as usize);
    let t = t
        .into_iter()
        .rev()
        .fold(0usize, |s, x| (s << 1) | (x == 'B') as usize);

    let mut q = VecDeque::new();
    let mut visited = vec![vec![false; n + 1]; 1 << (n + 2)];

    q.push_back((s, n, 0usize));
    visited[s][n] = true;

    while let Some((x, i, c)) = q.pop_front() {
        if x == t && i == n {
            println!("{c}");
            return;
        }
        (0..=n)
            .filter(|&j| j != i && j != i + 1 && j + 1 != i)
            .for_each(|j| {
                let y = (x >> j) & 3;

                let next = (x & !((1 << j) | (1 << (j + 1)))) | (y << i);

                if !visited[next][j] {
                    visited[next][j] = true;
                    q.push_back((next, j, c + 1));
                }
            });
    }

    println!("-1");
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
