fn main() {
    input! {
        n: usize, k: usize, x: usize,
        t: [usize; n],
    };

    let table = (0..=n)
        .map(|r| {
            (0..=(r + k - 1) / k)
                .map(|i| x * (i * r - k * i * i.saturating_sub(1) / 2))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut tt = t.clone();
    tt.dedup();
    let get = |ii: usize| {
        if ii == 0 { 0 } else { tt[ii - 1] }
    };

    let mut init = vec![vec![vec![usize::MAX; n + 1]; 2]];
    init[0][0][0] = 0;
    init[0][1][0] = 0;

    let dp = t
        .iter()
        .copied()
        .dedup_with_count()
        .enumerate()
        .fold(init, |prev, (i, (l, z))| {
            let mut next = vec![vec![vec![usize::MAX; n + 1]; 2]; i + 2];

            for j in 0..=i {
                for b in 0..2 {
                    for r in 0..n {
                        let y = prev[j][b][r];
                        if y == usize::MAX {
                            continue;
                        }

                        if b == 0 {
                            let last = get(j) + (get(i) - get(j)) / x * x;

                            if get(i + 1) - last >= x || last == 0 {
                                // 時刻zに出荷
                                let last2 =
                                    get(j) + ((get(i + 1) - get(j)) / x * x).saturating_sub(x);
                                assert!(last <= last2);

                                let num = ((last2 - last) / x).min((r + k - 1) / k);
                                let rr0 = r.saturating_sub(num * k);
                                let rr1 = (rr0 + l).saturating_sub(k);

                                next[i + 1][0][rr1] = min(
                                    next[i + 1][0][rr1],
                                    y.saturating_add(
                                        table[r][num] + rr0 * (z - last2) - (r * (get(i) - last)),
                                    ),
                                );
                            }

                            if (get(i + 1) - get(j)) % x != 0 {
                                // 手前まで毎回出荷
                                let last2 = get(j) + (get(i + 1) - get(j)) / x * x;
                                assert!(last <= last2);

                                let num = ((last2 - last) / x).min((r + k - 1) / k);
                                let rr0 = r.saturating_sub(num * k);
                                let rr1 = rr0 + l;

                                next[j][0][rr1] = min(
                                    next[j][0][rr1],
                                    y.saturating_add(
                                        table[r][num] + rr0 * (get(i + 1) - last2)
                                            - (r * (get(i) - last)),
                                    ),
                                );
                            }

                            if last + x <= z {
                                // 最後の出荷をしない
                                let last2 = get(j) + (get(i + 1) - get(j)) / x * x - x;
                                assert!(last <= last2);

                                let num = ((last2 - last) / x).min((r + k - 1) / k);

                                let rr0 = r.saturating_sub(num * k);
                                let rr1 = rr0 + l;

                                next[j][1][rr1] = min(
                                    next[j][1][rr1],
                                    y.saturating_add(
                                        table[r][num] + rr0 * (get(i + 1) - last2)
                                            - (r * (get(i) - last)),
                                    ),
                                );
                            }
                        } else if get(i) - get(j) >= x {
                            let last = get(j) + (get(i) - get(j)) / x * x - x;

                            if last + 2 * x > z {
                                let rr0 = r;
                                let rr1 = (rr0 + l).saturating_sub(k);

                                next[i + 1][0][rr1] = min(
                                    next[i + 1][0][rr1],
                                    y.saturating_add(rr0 * (get(i + 1) - get(i))),
                                );
                            }

                            if last + 2 * x > z {
                                let rr0 = r;
                                let rr1 = rr0 + l;

                                next[j][1][rr1] = min(
                                    next[j][1][rr1],
                                    y.saturating_add(rr0 * (get(i + 1) - get(i))),
                                );
                            }
                        } else if j == 0 {
                            {
                                let rr0 = r;
                                let rr1 = (rr0 + l).saturating_sub(k);

                                next[i + 1][0][rr1] = min(
                                    next[i + 1][0][rr1],
                                    y.saturating_add(rr0 * (get(i + 1) - get(i))),
                                );
                            }

                            {
                                let rr0 = r;
                                let rr1 = rr0 + l;

                                next[j][1][rr1] = min(
                                    next[j][1][rr1],
                                    y.saturating_add(rr0 * (get(i + 1) - get(i))),
                                );
                            }
                        } else {
                            assert_eq!(y, usize::MAX, "#{j} {b} {r}");
                        }
                    }
                }
            }

            next
        });

    let ans = dp
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row[0]
                .iter()
                .enumerate()
                .map(|(r, &y)| {
                    let last = get(i) + (*tt.last().unwrap() - get(i)) / x * x;

                    y.saturating_add(
                        table[r][(r + k - 1) / k].saturating_sub((*tt.last().unwrap() - last) * r),
                    )
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

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
