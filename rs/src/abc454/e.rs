fn main() {
    input! {
        t: usize,
        cases: [(usize, Usize1, Usize1); t],
    };

    cases
        .into_iter()
        .map(|(n, a, b)| {
            if n % 2 > 0 {
                None
            } else if (a + b) % 2 == 0 {
                None
            } else {
                let (aa, bb, sw) = if a % 2 == 0 {
                    (a, b, false)
                } else {
                    (b, a, true)
                };

                Some(
                    (0..aa)
                        .map(|i| {
                            if i % 2 == 0 {
                                (0..n).map(|j| (i, j)).collect::<Vec<_>>()
                            } else {
                                (0..n).rev().map(|j| (i, j)).collect::<Vec<_>>()
                            }
                        })
                        .chain(once(
                            (0..bb)
                                .flat_map(|j| {
                                    if j % 2 == 0 {
                                        [(aa, j), (aa + 1, j)]
                                    } else {
                                        [(aa + 1, j), (aa, j)]
                                    }
                                })
                                .chain(once((aa + 1, bb)))
                                .chain((bb + 1..n).flat_map(|j| {
                                    if j % 2 == 0 {
                                        [(aa + 1, j), (aa, j)]
                                    } else {
                                        [(aa, j), (aa + 1, j)]
                                    }
                                }))
                                .collect::<Vec<_>>(),
                        ))
                        .chain((aa + 2..n).map(|i| {
                            if i % 2 == 0 {
                                (0..n).rev().map(|j| (i, j)).collect::<Vec<_>>()
                            } else {
                                (0..n).map(|j| (i, j)).collect::<Vec<_>>()
                            }
                        }))
                        .flatten()
                        .map(|(i, j)| if sw { (j, i) } else { (i, j) })
                        .tuple_windows()
                        .map(|((i, j), (ii, jj))| {
                            if i == ii {
                                if j + 1 == jj { 'R' } else { 'L' }
                            } else if i + 1 == ii {
                                'D'
                            } else {
                                'U'
                            }
                        })
                        .collect::<Vec<_>>(),
                )
            }
        })
        .for_each(|ans| {
            if let Some(ans) = ans {
                println!("Yes");
                println!("{}", ans.iter().join(""));
            } else {
                println!("No");
            }
        });
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
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
use rand::{Rng, SeedableRng, rngs::SmallRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
