fn main() {
    input! {
        n: usize, m: usize, q: usize,
        s: [Digits; n],
        t: [Digits; q],
    };

    let table = (0..m)
        .map(|i| {
            (0..m)
                .map(|j| {
                    s.iter()
                        .map(|ss| (ss[i], ss[j]))
                        .fold([[false; 2]; 2], |mut seen, (x, y)| {
                            seen[x][y] = true;
                            seen
                        })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    t.iter()
        .map(|tt| iproduct!(0..m, 0..m).all(|(i, j)| table[i][j][tt[i]][tt[j]]))
        .for_each(|ans| {
            if ans {
                println!("Yes");
            } else {
                println!("No");
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
