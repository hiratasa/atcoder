fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let s = a.iter().sum::<usize>();
    if s % n > 0 {
        println!("-1");
        return;
    }

    let m = s / n;

    let b = a.iter().map(|&x| x as i64 - m as i64).collect::<Vec<_>>();

    let dp = (1..1 << n).fold(
        vec![(0, None); 1 << n],
        |mut dp: Vec<(usize, Option<(usize, bool)>)>, s| {
            let x = (0..n)
                .filter(|&i| s & (1 << i) > 0)
                .map(|i| b[i])
                .sum::<i64>()
                != 0;

            dp[s] = (0..n)
                .filter(|&i| s & (1 << i) > 0)
                .map(|i| (dp[s ^ (1 << i)].0.saturating_add(x as usize), Some((i, x))))
                .min()
                .unwrap();

            dp
        },
    );

    let mut b = b;
    let ans = successors(Some((1 << n) - 1), |&s| dp[s].1.map(|(i, _)| s ^ (1 << i)))
        .filter(|&s| dp[s].1.map_or(true, |(_, x)| !x))
        .tuple_windows()
        .map(|(s, t)| s ^ t)
        .flat_map(|s| {
            let mut pos = (0..n)
                .filter(|&i| s & (1 << i) > 0)
                .filter(|&i| b[i] > 0)
                .map(|i| (-b[i], i))
                .collect::<BinaryHeap<_>>();
            let mut neg = (0..n)
                .filter(|&i| s & (1 << i) > 0)
                .filter(|&i| b[i] < 0)
                .map(|i| (b[i], i))
                .collect::<BinaryHeap<_>>();

            let mut ops = vec![];
            while let Some(&(x, i)) = pos.peek()
                && let Some(&(y, j)) = neg.peek()
            {
                if x.abs() != b[i].abs() {
                    pos.pop();
                    continue;
                }
                if y.abs() != b[j].abs() {
                    neg.pop();
                    continue;
                }

                pos.pop();
                neg.pop();
                if b[i].abs() <= b[j].abs() {
                    ops.push((i, j, b[i]));
                    b[j] += b[i];
                    if b[j] != 0 {
                        neg.push((b[j], j));
                    }
                } else {
                    ops.push((i, j, -b[j]));
                    b[i] += b[j];
                    if b[i] != 0 {
                        pos.push((-b[i], i));
                    }
                }
            }

            ops
        })
        .collect::<Vec<_>>();

    println!("{}", ans.len());
    for (i, j, x) in ans {
        println!("{} {} {}", i + 1, j + 1, x);
    }
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
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
use rustc_hash::{FxHashMap, FxHashSet};
