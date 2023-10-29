#[allow(unused_imports)]
use std::{cmp::*, collections::*, f64, i64, io, iter::*, mem::*, str::*, usize};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom, Rng, SeedableRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};

#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
    source::{Readable, Source},
};

// vec with some initial value
#[allow(unused_macros)]
macro_rules! vvec {
    ($($x:expr),+; $y:expr; $n:expr) => {{
        let mut v = vec![$y; $n];

        let mut it = v.iter_mut();
        $(
            *it.next().unwrap() = $x;
        )+

        v
    }}
}

#[allow(unused_macros)]
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        if $n > 0 {
            bs.buffer_mut()[0] = $x as u64;
        }
        bs
    }};
}

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

use easy_ext::ext;

#[ext(IterCopyExt)]
impl<'a, I, T> I
where
    Self: IntoIterator<Item = &'a T>,
    T: 'a + Copy,
{
    fn citer(self) -> std::iter::Copied<Self::IntoIter> {
        self.into_iter().copied()
    }
}

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

fn main() {
    input! {
        n: usize, m: usize,
        p: [Usize1; n + m]
    }

    let [c0, c1, c2] = p
        .citer()
        .scan(vec![false; n + m], |seen, i| {
            if !seen[i] {
                let (has1, has2, len) = iterate(i, |&j| p[j])
                    .take_while(|&j| !replace(&mut seen[j], true))
                    .fold((false, false, 0usize), |(has1, has2, len), j| {
                        if j < n {
                            (true, has2, len + 1)
                        } else {
                            (has1, true, len + 1)
                        }
                    });

                match (has1, has2, len) {
                    (_, _, 1) => Some(Some(0)),
                    (true, true, _) => Some(Some(0)),
                    (true, false, _) => Some(Some(1)),
                    (false, true, _) => Some(Some(2)),
                    (false, false, _) => unreachable!(),
                }
            } else {
                Some(None)
            }
        })
        .flatten()
        .fold([0; 3], |mut c, idx| {
            c[idx] += 1;
            c
        });

    let ans = n + m - c0 + c1.abs_diff(c2);

    println!("{ans}");
}
