fn main() {
    input! {
        n: usize, m: usize,
        s: [Digits; n],
    };

    if m <= 14 {
        let s = s
            .into_iter()
            .map(|x| x.into_iter().fold(0usize, |a, b| 2 * a + b))
            .collect::<Vec<_>>();
        let mask = (1 << m) - 1;
        let ans = (0usize..1 << m).find(|&t| {
            s.iter()
                .copied()
                .all(|x| (x & t) > 0 || (!x & !t) & mask > 0)
        });

        if let Some(ans) = ans {
            println!("Yes");
            println!("{}", (0..m).map(|i| (ans >> (m - 1 - i)) & 1).join(""));
        } else {
            println!("No");
        }
    } else {
        let ans = (0..m)
            .scan((vec![false; n], n), |(covered, rest), i| {
                let n1 = (0..n).filter(|&j| !covered[j] && s[j][i] > 0).count();

                let b = if 2 * n1 >= *rest { 1 } else { 0 };

                for j in 0..n {
                    if s[j][i] == b && !covered[j] {
                        covered[j] = true;
                        *rest -= 1;
                    }
                }

                Some(b)
            })
            .collect::<Vec<_>>();

        println!("Yes");
        println!("{}", ans.iter().join(""));
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
