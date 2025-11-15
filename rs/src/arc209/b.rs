fn main() {
    input! {
        t: usize,
        cases: [Chars; t],
    };

    cases
        .into_iter()
        .map(|s| {
            let n = s.len();

            let freq = s.into_iter().fold(vec![0; 26], |mut freq, c| {
                freq[c as usize - 'a' as usize] += 1;
                freq
            });

            let ordered = (0..26).sorted_by_key(|&i| freq[i]).collect::<Vec<_>>();

            if freq[ordered[25]] <= (n + 1) / 2 {
                (0..n)
                    .step_by(2)
                    .chain((1..n).step_by(2))
                    .zip(ordered.into_iter().rev().flat_map(|i| repeat_n(i, freq[i])))
                    .fold(vec!['a'; n], |mut ans, (pos, c)| {
                        ans[pos] = (c as u8 + 'a' as u8) as char;
                        ans
                    })
            } else {
                let c = ordered[25];
                let k = freq[c];
                let m = n - k + 1;
                let q = k / m;
                let q = if q % 2 == 0 { q - 1 } else { q };

                (0..m)
                    .scan(k - m * q, |r, i| {
                        if *r >= 2 {
                            *r -= 2;
                            Some(q + 2)
                        } else if *r > 0 && i == m - 1 {
                            Some(q + 1)
                        } else {
                            Some(q)
                        }
                    })
                    .map(|x| Ok(x))
                    .interleave(
                        (0..26)
                            .filter(|&i| i != c)
                            .flat_map(|i| repeat_n(Err(i), freq[i])),
                    )
                    .flat_map(|res| match res {
                        Ok(l) => repeat_n(c, l),
                        Err(i) => repeat_n(i, 1),
                    })
                    .map(|c| (c as u8 + 'a' as u8) as char)
                    .collect::<Vec<_>>()
            }
        })
        .for_each(|ans| {
            println!("{}", ans.iter().join(""));
        });
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
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
use rustc_hash::{FxHashMap, FxHashSet};
