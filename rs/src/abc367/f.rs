fn main() {
    input! {
        n: usize, q: usize,
        a: [usize; n],
        b: [usize; n],
        queries: [[(Usize1, usize); 2]; q],
    };

    let mut rng = SmallRng::seed_from_u64(42);

    let counts = [&a, &b]
        .into_iter()
        .map(|v| {
            v.iter().fold(vec![0; n + 1], |mut counts, &x| {
                counts[x] += 1;
                counts
            })
        })
        .collect::<Vec<_>>();
    let counts_max = izip!(&counts[0], &counts[1])
        .map(|(&x, &y)| max(x, y))
        .collect::<Vec<_>>();

    let values = (0..=n)
        .map(|i| {
            (0..counts_max[i])
                .map(|_| rng.gen::<usize>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let block = (n as f64 / (q as f64).sqrt()).ceil() as usize;

    let hashes = (0..2)
        .map(|idx| {
            let v = if idx == 0 { &a } else { &b };
            queries
                .iter()
                .map(|query| query[idx])
                .enumerate()
                .sorted_by_key(|&(_, (l, r))| (r / block, l))
                .scan(
                    (0, 0, 0, vec![0; n + 1]),
                    |(h, ll, rr, nums), (i, (l, r))| {
                        while l < *ll {
                            *ll -= 1;
                            let x = v[*ll];
                            *h ^= values[x][nums[x]];
                            nums[x] += 1;
                        }

                        while *rr < r {
                            let x = v[*rr];
                            *rr += 1;
                            *h ^= values[x][nums[x]];
                            nums[x] += 1;
                        }

                        while *ll < l {
                            let x = v[*ll];
                            *ll += 1;
                            nums[x] -= 1;
                            *h ^= values[x][nums[x]];
                        }

                        while r < *rr {
                            *rr -= 1;
                            let x = v[*rr];
                            nums[x] -= 1;
                            *h ^= values[x][nums[x]];
                        }

                        Some((i, *h))
                    },
                )
                .sorted()
                .map(|(_, h)| h)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (0..q)
        .map(|i| hashes[0][i] == hashes[1][i])
        .for_each(|ans| {
            if ans {
                println!("Yes");
            } else {
                println!("No");
            }
        });
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
use rand::{rngs::SmallRng, Rng, SeedableRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
