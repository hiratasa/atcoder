fn main() {
    input! {
        t: usize,
        cases: [(usize, Digits); t],
    };

    cases
        .into_iter()
        .map(|(_, s)| {
            let n1 = s.iter().copied().filter(|&d| d == 1).count() as i64;
            s.into_iter()
                .scan((0, 0), |(ma, cur), d| {
                    if d == 1 {
                        *cur -= 1;
                    } else {
                        *cur += 1;
                    }
                    *ma = max(*ma, *cur);

                    Some(n1 + *cur - *ma)
                })
                .min()
                .unwrap()
        })
        .for_each(|ans| {
            println!("{ans}");
        })
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
