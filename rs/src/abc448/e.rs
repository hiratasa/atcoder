fn main() {
    const L: usize = 10007;

    input! {
        k: usize, m: usize,
        cl: [(usize, usize); k],
    };

    let t = (0..10)
        .map(|c| {
            let mut t = vec![vec![(0, 0); m]; 30];
            t[0] = (0..m)
                .map(|r| {
                    let x = 10 * r + c;

                    (x / m % L, x % m)
                })
                .collect::<Vec<_>>();

            for i in 1..30 {
                for j in 0..m {
                    let (q, r) = t[i - 1][j];
                    t[i][j] = (
                        (q * pow_mod(10, 1usize << (i - 1), L) + t[i - 1][r].0) % L,
                        t[i - 1][r].1,
                    );
                }
            }

            t
        })
        .collect::<Vec<_>>();

    let ans = cl
        .into_iter()
        .fold((0, 0), |(mut q, mut r), (c, mut l)| {
            for i in (0..30).rev() {
                if l >= 1usize << i {
                    (q, r) = (
                        (q * pow_mod(10, 1usize << i, L) + t[c][i][r].0) % L,
                        t[c][i][r].1,
                    );
                    l -= 1usize << i;
                }
            }
            assert_eq!(l, 0);

            (q, r)
        })
        .0;

    println!("{ans}");
}

pub fn pow_mod(mut x: usize, mut p: usize, m: usize) -> usize {
    let mut y = 1;

    x = x % m;
    while p > 0 {
        if p & 1 > 0 {
            y = y * x % m;
        }

        x = x * x % m;
        p >>= 1;
    }

    y
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
