fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let mut ans = vec![vec!['0'; n]; n];
    for t in 0..=2 * (n - 1) {
        let x = (0..=t)
            .map(|i| (i, t - i))
            .filter(|&(i, j)| i < n && j < n)
            .map(|(i, j)| s[i][j])
            .find(|&c| c != '?')
            .unwrap_or('0');

        if (0..=t)
            .map(|i| (i, t - i))
            .filter(|&(i, j)| i < n && j < n)
            .map(|(i, j)| s[i][j])
            .all(|c| c == x || c == '?')
        {
            (0..=t)
                .map(|i| (i, t - i))
                .filter(|&(i, j)| i < n && j < n)
                .for_each(|(i, j)| {
                    ans[i][j] = x;
                });
        } else {
            println!("-1");
            return;
        }
    }

    for row in ans {
        println!("{}", row.iter().join(""));
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
