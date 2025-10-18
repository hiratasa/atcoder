#[allow(unused_imports)]
use std::{cmp::*, collections::*, f64, i64, io, iter::*, mem::*, str::*, usize};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom};
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

fn solve0(ab: &mut [(usize, usize)]) -> usize {
    if ab.citer().all(|(x, y)| x == y) {
        return 0;
    }

    let n = ab.len();

    (0..n)
        .map(|i| {
            if ab[i].0 > 0 {
                ab[i].0 -= 1;

                let r = (0..n)
                    .map(|j| {
                        if ab[j].1 > 0 {
                            ab[j].1 -= 1;
                            let r = solve0(ab);
                            ab[j].1 += 1;
                            r + 1
                        } else {
                            usize::MAX
                        }
                    })
                    .min()
                    .unwrap();
                ab[i].0 += 1;
                r
            } else {
                0
            }
        })
        .max()
        .unwrap()
}

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    }

    let s = ab.citer().map(|(a, _)| a).sum::<usize>();
    let ans = ab
        .citer()
        .filter(|&(x, y)| x > y)
        .map(|(_, y)| y)
        .min()
        .map_or(0, |y| s - y);

    println!("{ans}");
}
