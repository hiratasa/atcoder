fn main() {
    input! {
        n: usize,
        a: [Digits; 3],
    };

    let positions = a
        .iter()
        .map(|s| s.iter().copied().positions(|x| x == 1).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (ans, c) = (0..n).fold((vec![0; 2 * n], 0), |(mut ans, c), i| {
        let pos = positions.iter().map(|p| p[i]).sorted().nth(1).unwrap();
        let d = positions.iter().map(|p| p[i].abs_diff(pos)).sum::<usize>();
        ans[pos] = 1;
        (ans, c + d)
    });

    println!("{c}");
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
