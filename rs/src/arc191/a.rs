fn main() {
    input! {
        n: usize, m: usize,
        mut s: Digits,
        t: Digits,
    };

    let mut freq = t.iter().copied().fold(vec![0usize; 10], |mut freq, x| {
        freq[x] += 1;
        freq
    });

    let mut used = vec![false; 10];
    let mut x = 9;
    for i in 0..n {
        while x > 0 && freq[x] == 0 {
            x -= 1;
        }

        if s[i] >= x {
            used[s[i]] = true;
            continue;
        }

        if freq[x] > 0 {
            s[i] = x;
            freq[x] -= 1;
            used[x] = true;
        }
    }

    if !used[t[m - 1]] {
        s[n - 1] = t[m - 1];
    }

    println!("{}", s.iter().join(""));
}

fn solve0(s: &[usize], t: &[usize]) -> Vec<usize> {
    let n = s.len();
    let m = t.len();

    (0..m)
        .map(|_| 0..n)
        .multi_cartesian_product()
        .map(|p| {
            let mut s = s.to_vec();

            for (pos, &x) in p.into_iter().zip(t) {
                s[pos] = x;
            }

            s
        })
        .max()
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
use rand::{rngs::SmallRng, Rng, SeedableRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};

use proconio::source::{Readable, Source};
enum Digits {}
impl Readable for Digits {
    type Output = Vec<usize>;
    fn read<R: std::io::BufRead, S: Source<R>>(source: &mut S) -> Vec<usize> {
        source
            .next_token_unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
    }
}
