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

fn main() {
    input! {
        n: usize, k: usize,
        mut a: [usize; n]
    }

    a.sort();

    let ans = a
        .citer()
        .try_fold((BTreeSet::<usize>::new(), 1), |(prev_holes, c), x| {
            if prev_holes.range(..x).count() + x.saturating_sub(c) >= k {
                return Err(prev_holes
                    .range(..x)
                    .copied()
                    .chain(c..max(c, x))
                    .take(k)
                    .collect::<Vec<_>>());
            }

            let mut next_holes = chain(
                prev_holes.range(..x).copied(),
                prev_holes
                    .citer()
                    .map(|y| y + x)
                    .filter(|&y| prev_holes.contains(&y) || y >= c),
            )
            .collect::<BTreeSet<_>>();
            if c <= x {
                next_holes.extend(c..x);
            }

            Ok((next_holes, c + x))
        })
        .map_or_else(
            |ans| ans,
            |(holes, c)| holes.into_iter().chain(c..).take(k).collect::<Vec<_>>(),
        );

    println!("{}", ans.citer().join(" "));
}
