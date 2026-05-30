fn main() {
    input! {
        t: usize,
        cases: [usize; t],
    };

    let mut memo = FxHashMap::default();

    for n in cases {
        if calc(n, &mut memo) {
            println!("Yes");
            let ans = &memo[&n];
            println!("{}", ans.iter().join(" "));
        } else {
            println!("No");
        }
    }
}

fn calc(n: usize, memo: &mut FxHashMap<usize, Vec<usize>>) -> bool {
    if let Some(_x) = memo.get(&n) {
        return true;
    }

    if n == 1 {
        memo.insert(n, vec![1]);
        return true;
    }

    if n <= 3 || n == 5 {
        return false;
    }

    if n < 36 {
        let perm = [2usize, 3, 4, 6, 9]
            .into_iter()
            .combinations_with_replacement(n)
            .find(|perm| {
                let p = perm
                    .iter()
                    .copied()
                    .map(|x| x * x)
                    .fold(1usize, |x, y| x / gcd(x, y) * y);

                perm.iter()
                    .copied()
                    .map(|x| x * x)
                    .map(|x| p / x)
                    .sum::<usize>()
                    == p
            })
            .unwrap();

        memo.insert(n, perm);

        return true;
    }

    let r = n % 6;
    let q = n / 6;

    calc(6, memo);
    calc(6 + r, memo);
    calc(q, memo);

    let s = memo[&q]
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            if i == 0 {
                memo[&(6 + r)].iter().map(|&y| x * y).collect::<Vec<_>>()
            } else {
                memo[&6].iter().map(|&y| x * y).collect::<Vec<_>>()
            }
        })
        .collect::<Vec<_>>();
    memo.insert(n, s);
    true
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 { b } else { gcd(b % a, a) }
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
