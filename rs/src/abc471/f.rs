fn main() {
    input! {
        n: usize, k: usize,
        s: [Digits; n],
    };

    let mut words = s.iter().fold(vec![vec![]; 11], |mut words, x| {
        words[x.len()].push(x);
        words
    });

    words
        .iter_mut()
        .for_each(|t| t.sort_by_key(|x| Reverse(*x)));

    let a = 10
        - (0..=10)
            .rev()
            .map(|i| words[i].len())
            .cumsum::<usize>()
            .position(|x| x >= k)
            .unwrap();
    let l = words[a + 1..].iter().map(|w| w.len()).sum::<usize>();

    let mut candidates = vec![];
    for i in 1..=10 {
        let Some(w) = words[i].first().copied() else {
            continue;
        };

        let words = &words;
        let ans = w
            .iter()
            .copied()
            .chain(
                (a..=10)
                    .flat_map(move |j| {
                        let skip = if j == i { 1 } else { 0 };

                        let take = if j == a && i >= a {
                            k - l
                        } else if j == a {
                            k - 1 - l
                        } else {
                            words[j].len()
                        };

                        words[j].iter().copied().take(take).skip(skip)
                    })
                    .sorted_by(|x, y| {
                        x.iter()
                            .copied()
                            .chain(y.iter().copied())
                            .zip(y.iter().copied().chain(x.iter().copied()))
                            .find_map(|(c, d)| if c != d { Some(c.cmp(&d)) } else { None })
                            .unwrap_or(Ordering::Equal)
                    })
                    .rev()
                    .flatten()
                    .copied(),
            )
            .skip_while(|&c| c == 0)
            .collect::<Vec<_>>();

        candidates.push(ans);
    }

    candidates.push(vec![0]);
    let ans = candidates.iter().max_by_key(|x| (x.len(), *x)).unwrap();

    println!("{}", ans.iter().join(""));
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
