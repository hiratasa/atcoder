fn main() {
    input! {
        l: usize, r: usize,
    };

    const M: usize = 10000000;

    let primes = (2..=M)
        .scan(vec![true; M + 1], |is_prime, p| {
            if is_prime[p] {
                for i in (2..).map(|i| i * p).take_while(|&i| i <= M) {
                    is_prime[i] = false;
                }
                Some(Some(p))
            } else {
                Some(None)
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    let ans0 = primes
        .iter()
        .map(|&p| {
            let x = iterate(1usize, |&x| x.saturating_mul(p))
                .position(|x| x > l)
                .unwrap()
                - 1;
            let y = iterate(1usize, |&x| x.saturating_mul(p))
                .position(|x| x > r)
                .unwrap()
                - 1;

            y - x
        })
        .sum::<usize>();

    let mut t = vec![true; r - l + 1];
    for &p in &primes {
        ((l - 1) / p + 1..=(r / p)).for_each(|i| {
            t[i * p - l] = false;
        });
    }

    let ans1 = t.iter().skip(1).filter(|&&x| x).count();

    let ans = 1 + ans0 + ans1;

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
