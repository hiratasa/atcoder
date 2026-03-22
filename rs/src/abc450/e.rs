fn main() {
    input! {
        x: Chars,
        y: Chars,
        q: usize,
        lrc: [(Usize1, usize, char); q],
    };

    let calc_fib = |n: usize, m: usize| {
        once(n)
            .chain(
                successors(Some((n, m)), |&(s, t)| Some((t, s.checked_add(t)?)))
                    .take(100)
                    .map(|(s, t)| t),
            )
            .collect::<Vec<_>>()
    };

    let freq_x = x.iter().copied().fold(vec![0; 26], |mut freq, c| {
        let c = c as usize - 'a' as usize;
        freq[c] += 1;
        freq
    });
    let freq_y = y.iter().copied().fold(vec![0; 26], |mut freq, c| {
        let c = c as usize - 'a' as usize;
        freq[c] += 1;
        freq
    });

    let lens = calc_fib(x.len(), y.len());
    let table = (0..26)
        .map(|i| calc_fib(freq_x[i], freq_y[i]))
        .collect::<Vec<_>>();

    let nums_x = (0..26)
        .map(|i| {
            once(0)
                .chain(
                    x.iter()
                        .copied()
                        .map(|c| (c as usize == 'a' as usize + i) as usize)
                        .cumsum::<usize>(),
                )
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let nums_y = (0..26)
        .map(|i| {
            once(0)
                .chain(
                    y.iter()
                        .copied()
                        .map(|c| (c as usize == 'a' as usize + i) as usize)
                        .cumsum::<usize>(),
                )
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let count_in_prefix = |mut i: usize, c: usize, mut prefix_len: usize| {
        let mut ret = 0;
        while i > 1 {
            if prefix_len <= lens[i - 1] {
                i -= 1;
            } else {
                ret += table[c][i - 1];
                prefix_len -= lens[i - 1];
                i -= 2;
            }
            // eprintln!("{} {} {}", prefix_len, lens[i], i);
        }

        if i == 1 {
            ret + nums_y[c][prefix_len]
        } else {
            ret + nums_x[c][prefix_len]
        }
    };

    lrc.into_iter()
        .map(|(l, r, c)| {
            let c = c as usize - 'a' as usize;

            count_in_prefix(lens.len(), c, r) - count_in_prefix(lens.len(), c, l)
        })
        .for_each(|ans| {
            println!("{ans}");
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
