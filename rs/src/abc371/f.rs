fn main() {
    input! {
        n: usize,
        x: [i64; n],
        queries: [(Usize1, i64)],
    };

    let blocks = x
        .into_iter()
        .enumerate()
        .map(|(i, x)| (i, (x, 1)))
        .collect::<BTreeMap<_, _>>();
    let ans = queries
        .into_iter()
        .scan(blocks, |blocks, (t, g)| {
            let x = {
                let (&idx, &(x, m)) = blocks.range(..=t).next_back().unwrap();

                blocks.remove(&idx);

                let (x, m) = if idx < t {
                    blocks.insert(idx, (x, t - idx));
                    (x + (t - idx) as i64, m - (t - idx))
                } else {
                    (x, m)
                };
                if m > 1 {
                    blocks.insert(t + 1, (x + 1, m - 1));
                }

                x
            };

            match x.cmp(&g) {
                Ordering::Less => {
                    let mut m = 1;
                    let mut cost = (g - x) as usize * m;
                    while let Some((&idx, &(xx, mm))) = blocks.range(t + m..).next() {
                        if xx >= g + (idx - t) as i64 {
                            break;
                        } else {
                            blocks.remove(&idx);
                            m += mm;
                            cost += (g + (idx - t) as i64 - xx) as usize * mm;
                        }
                    }

                    blocks.insert(t, (g, m));
                    Some(cost)
                }
                Ordering::Equal => Some(0),
                Ordering::Greater => {
                    let mut st = t;
                    let mut m = 1;
                    let mut cost = (x - g) as usize * m;
                    while let Some((&idx, &(xx, mm))) = blocks.range(..t).next_back() {
                        if xx <= g - (t - idx) as i64 {
                            break;
                        } else {
                            blocks.remove(&idx);
                            st = idx;
                            m += mm;
                            cost += (xx - (g - (t - idx) as i64)) as usize * mm;
                        }
                    }

                    blocks.insert(st, (g - (t - st) as i64, m));
                    Some(cost)
                }
            }
        })
        .sum::<usize>();

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
