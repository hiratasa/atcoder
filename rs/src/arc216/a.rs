fn main() {
    input! {
        t: usize,
        cases: [(usize, Digits, Digits); t],
    };

    cases
        .into_iter()
        .map(|(n, a, b)| {
            if a[0] != b[0] || a[n - 1] != b[n - 1] {
                return None;
            }

            let c = a
                .iter()
                .copied()
                .tuple_windows()
                .map(|(x, y)| x == y)
                .enumerate()
                .map(|(i, x)| x ^ (i % 2 == 0))
                .collect::<Vec<_>>();
            let d = b
                .iter()
                .copied()
                .tuple_windows()
                .map(|(x, y)| x == y)
                .enumerate()
                .map(|(i, x)| x ^ (i % 2 == 0))
                .collect::<Vec<_>>();

            if c.iter().copied().filter(|&x| x).count() != d.iter().copied().filter(|&x| x).count()
            {
                return None;
            }

            Some(
                izip!(
                    c.iter().copied().positions(|x| x),
                    d.iter().copied().positions(|x| x)
                )
                .map(|(i, j)| i.abs_diff(j))
                .sum::<usize>(),
            )
        })
        .for_each(|ans| {
            if let Some(ans) = ans {
                println!("{ans}");
            } else {
                println!("-1");
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
