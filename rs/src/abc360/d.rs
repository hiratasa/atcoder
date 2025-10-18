fn main() {
    input! {
        n: usize, t: i64,
        s: Digits,
        x: [i64; n],
    };

    let idxs = (0..n).sorted_by_key(|&i| x[i]).collect::<Vec<_>>();

    let ans = (0..n)
        .scan((0, 0), |(j, k), i| {
            while *j < n && x[idxs[*j]] - x[idxs[i]] <= 2 * t {
                if s[idxs[*j]] == 0 {
                    *k += 1;
                }

                *j += 1;
            }

            if s[idxs[i]] == 0 {
                *k -= 1;
                Some(0)
            } else {
                Some(*k)
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
