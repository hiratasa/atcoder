fn main() {
    input! {
        s: Chars,
    };

    let s = s
        .iter()
        .copied()
        .scan(vec![], |seen, c| {
            if let Some(idx) = seen.iter().copied().position(|d| d == c) {
                Some(idx)
            } else {
                seen.push(c);
                Some(seen.len() - 1)
            }
        })
        .collect::<Vec<_>>();
    let n = s.iter().copied().max().unwrap() + 1;

    const M: usize = 10000000;
    let mut is_prime = (2..M)
        .take_while(|&x| x * x < M)
        .fold(vec![true; M], |mut is_prime, x| {
            if is_prime[x] {
                (2..)
                    .map(|y| x * y)
                    .take_while(|&y| y < M)
                    .for_each(|y| is_prime[y] = false);
            }

            is_prime
        });
    is_prime[1] = false;

    let ans = (0..10)
        .permutations(n)
        .filter(|perm| perm[s[0]] > 0)
        .map(|perm| {
            s.iter()
                .copied()
                .map(|d| perm[d])
                .fold(0, |x, y| 10 * x + y)
        })
        .find(|&p| is_prime[p]);

    if let Some(ans) = ans {
        println!("{ans}");
    } else {
        println!("-1");
    }
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
