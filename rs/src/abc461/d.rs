fn main() {
    input! {
        h: usize, w: usize, k: usize,
        s: [Digits; h],
    };

    let sums = once(vec![0; w + 1])
        .chain(
            s.into_iter()
                .map(|row| once(0).chain(row).cumsum::<usize>().collect::<Vec<_>>()),
        )
        .scan(vec![0; w + 1], |sums, row| {
            sums.iter_mut().zip(row).for_each(|(x, y)| *x += y);

            Some(sums.clone())
        })
        .collect::<Vec<_>>();

    let count = |k: usize| {
        (0..=h)
            .tuple_combinations()
            .map(|(i0, i1)| {
                (0..w)
                    .scan(1usize, |j1, j0| {
                        *j1 = max(*j1, j0 + 1);

                        while *j1 <= w
                            && sums[i1][*j1] + sums[i0][j0] - sums[i1][j0] - sums[i0][*j1] < k
                        {
                            *j1 += 1;
                        }

                        Some(*j1)
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    };

    let ans = count(k + 1) - count(k);

    println!("{ans}");
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
