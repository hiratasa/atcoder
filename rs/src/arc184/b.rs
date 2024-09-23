fn main() {
    input! {
        n: usize,
    };

    let mut ps = vec![];
    let mut dp: Vec<Vec<Vec<usize>>> = vec![];

    const OFFSET2: i64 = 30;
    for i2 in -30i64.. {
        let x = 2usize.pow(i2.abs() as u32);
        if i2 > 0 && x > n {
            break;
        }

        dp.push(vec![]);
        for i3 in 0.. {
            let y = 3usize.pow(i3 as u32);

            let (m0, m1) = if i2 < 0 { (x, y) } else { (1, x * y) };

            if m0 * n < m1 {
                break;
            }

            if m0 == 1 {
                ps.push((m1, i2, i3));
            }

            let k = (0..)
                .skip_while(|&k| 3usize.pow(k).saturating_mul(m0) <= m1)
                .next()
                .unwrap();
            let mask = (1usize << k) - 1;

            let mut t = vec![usize::MAX; 1 << k];

            if i2 + OFFSET2 == 0 {
                t[0] = 0;
            } else {
                let prev = &dp[(i2 + OFFSET2 - 1) as usize][i3].clone();
                for (s, &pp) in prev.iter().enumerate() {
                    let idx = (s | (s << 1)) & mask;
                    t[idx] = min(t[idx], pp.saturating_add(s.count_ones() as usize));
                }
            }

            for s in 0..1 << k {
                for i in 0..k {
                    let idx = (s | (1 << i) | (1 << (i + 1))) & mask;
                    t[idx] = min(t[idx], t[s].saturating_add(1));
                }
            }

            let mut t2 = vec![0; 1 << k];
            for s in 0..1 << k {
                t2[(!s) & mask] = t[s];
            }

            // eprintln!("#{i2},{i3}: {t2:?}");

            dp[(i2 + OFFSET2) as usize].push(t2);
        }
    }

    ps.sort();

    let mut idx = ps.len() - 1;
    let mut ans = 0;
    for i in 0.. {
        if 6 * i >= n {
            break;
        }

        for j in [1, 5] {
            let x = 6 * i + j;

            if x > n {
                break;
            }

            while ps[idx].0 * x > n {
                idx -= 1;
            }

            let (_, i2, i3) = ps[idx];

            ans += dp[(i2 + OFFSET2) as usize][i3][0];
        }
    }

    println!("{ans}");
}

#[allow(dead_code)]
fn solve0(n: usize) -> usize {
    (0usize..1 << n)
        .filter(|&s| {
            (1..=n)
                .filter(|&i| s & (1 << (i - 1)) > 0)
                .flat_map(|i| [i, 2 * i, 3 * i])
                .filter(|&x| x <= n)
                .sorted()
                .dedup()
                .eq(1..=n)
        })
        .map(|s| s.count_ones() as usize)
        .min()
        .unwrap()
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
