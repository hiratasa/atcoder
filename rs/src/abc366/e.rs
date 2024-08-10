fn main() {
    input! {
        n: usize, d: i64,
        xy: [(i64, i64); n],
    };

    let xs = xy.iter().map(|&t| t.0).sorted().collect::<Vec<_>>();
    let ys = xy.iter().map(|&t| t.1).sorted().collect::<Vec<_>>();

    const M: i64 = 2000010;

    let z0 = xs.iter().map(|&x| (x - (-M - 1)).abs()).sum::<i64>();

    let k0 = ys.iter().map(|&y| (ys[(n - 1) / 2] - y).abs()).sum::<i64>();
    let k1 = ys.iter().map(|&y| (ys[n / 2] - y).abs()).sum::<i64>();

    let i0 = (0..=(n - 1) / 2)
        .rev()
        .take_while(|&i| ys[i] == ys[(n - 1) / 2])
        .last()
        .unwrap();
    let i1 = ((n - 1) / 2..n)
        .take_while(|&i| ys[i] == ys[(n - 1) / 2])
        .last()
        .unwrap();

    let ans = (-M..=M)
        .scan((z0, 0), |(z, i), x| {
            *z += *i as i64 - (n - *i) as i64;
            while *i < n && xs[*i] == x {
                *i += 1;
            }

            Some(d - *z)
            // 3 => (1, 1)
            // 4 => (1, 2)
        })
        .scan(
            (k0, ys[(n - 1) / 2], i0, k1, ys[(n - 1) / 2], i1, i64::MIN),
            |(k0, y0, i0, k1, y1, i1, prev), r| {
                if *prev <= r {
                    while *k0 <= r {
                        *k0 += (n - *i0) as i64 - *i0 as i64;
                        *y0 -= 1;
                        while *i0 > 0 && ys[*i0 - 1] >= *y0 {
                            *i0 -= 1;
                        }
                    }
                    while *k1 <= r {
                        *k1 += *i1 as i64 + 1 - (n - *i1 - 1) as i64;
                        *y1 += 1;
                        while *i1 + 1 < n && ys[*i1 + 1] <= *y1 {
                            *i1 += 1;
                        }
                    }
                } else {
                    while *i0 < n && ys[*i0] == *y0 {
                        *i0 += 1;
                    }
                    *i1 = min(*i1 + 1, n);
                    while *i1 > 0 && ys[*i1 - 1] >= *y1 {
                        *i1 -= 1;
                    }

                    while *k0 + *i0 as i64 - (n - *i0) as i64 > r && *y0 <= *y1 {
                        *k0 += *i0 as i64 - (n - *i0) as i64;
                        *y0 += 1;
                        while *i0 < n && ys[*i0] == *y0 {
                            *i0 += 1;
                        }
                    }
                    while *k1 + (n - *i1) as i64 - *i1 as i64 > r && *y0 <= *y1 {
                        *k1 += (n - *i1) as i64 - *i1 as i64;
                        *y1 -= 1;
                        while *i1 > 0 && ys[*i1 - 1] == *y1 {
                            *i1 -= 1;
                        }
                    }
                }

                *prev = r;

                Some((*y1 - *y0 - 1).max(0))
            },
        )
        .sum::<i64>();

    println!("{ans}");
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
