fn main() {
    input! {
        n: usize, k: usize,
        p: [Usize1; n * k],
    };

    let mut used = vec![false; n * k];
    let mut cycles = vec![];
    for i in 0..n * k {
        if used[i] {
            continue;
        }

        let mut j = p[i];
        let mut cycle = vec![i % n];
        used[i] = true;
        while j != i {
            cycle.push(j % n);
            used[j] = true;
            j = p[j];
        }
        cycles.push(cycle);
    }

    let ans = cycles
        .into_iter()
        .map(|cycle| solve(&cycle, n) as usize)
        .sum::<usize>();

    println!("{ans}");
}

fn solve(cycle: &[usize], n: usize) -> u32 {
    let l = cycle.len();
    let (nexts, _) = (0..2 * l).fold(
        (vec![None; 2 * l], vec![None; n]),
        |(mut nexts, mut lasts), i| {
            if let Some(j) = lasts[cycle[i % l]] {
                nexts[j] = Some(i);
            }
            lasts[cycle[i % l]] = Some(i);
            (nexts, lasts)
        },
    );

    let mut dp = vec![0u32; l * l];
    for i in (0..l).rev() {
        for len in 2..=l {
            let j = i + len - 1;
            if j >= 2 * l {
                continue;
            }

            let ii = i;
            let jj = j % l;
            if cycle[ii] == cycle[jj] {
                if len == 2 {
                    dp[i + jj * l] = 1;
                } else {
                    dp[i + jj * l] = dp[(i + 1) + (j - 1) % l * l] + 1;
                }
            }

            dp[i + jj * l] = max(dp[i + jj * l], dp[(i + 1) % l + jj * l]);
            let mut x = i;
            while let Some(x2) = nexts[x] {
                x = x2;
                let xx = if x >= l { x - l } else { x };
                if x >= j {
                    break;
                }
                dp[i + jj * l] = max(dp[i + jj * l], dp[i + xx * l] + dp[xx + jj * l]);
            }
        }
    }

    (0..l).map(|i| dp[i + (i + l - 1) % l * l]).max().unwrap()
}

fn solve0(cycle: &[usize], n: usize) -> usize {
    let l = cycle.len();
    let mut ret = 0;
    for i in 0..l {
        for j in i + 1..l {
            if cycle[i] == cycle[j] {
                let a0 = solve0(&cycle[i + 1..=j], n);
                let cycle2 = cycle[j + 1..]
                    .iter()
                    .chain(cycle[..=i].iter())
                    .copied()
                    .collect::<Vec<_>>();
                let a1 = solve0(&cycle2, n);
                ret = max(ret, a0 + a1 + 1);
            }
        }
    }

    ret
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
use rand::{rngs::SmallRng, seq::SliceRandom, Rng, SeedableRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
