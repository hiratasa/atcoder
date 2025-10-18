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
        n: usize,
        a: [f64; n]
    }

    let b = a
        .citer()
        .map(|x| (x * 1e9).round() as usize)
        .map(|x| {
            (
                x.trailing_zeros() as usize,
                iterate(x, |&y| y / 5).position(|y| y % 5 > 0).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    const M: usize = 60;
    let mut table = b.citer().fold(vec![vec![0; M]; M], |mut table, (i, j)| {
        table[i][j] += 1;
        table
    });

    for i in (0..M - 1).rev() {
        for j in 0..M {
            table[i][j] += table[i + 1][j];
        }
    }
    for i in 0..M {
        for j in (0..M - 1).rev() {
            table[i][j] += table[i][j + 1];
        }
    }

    let ans = (b
        .citer()
        .map(|(i, j)| table[18usize.saturating_sub(i)][18usize.saturating_sub(j)])
        .sum::<usize>()
        - b.citer().filter(|&(i, j)| i >= 9 && j >= 9).count())
        / 2;

    println!("{ans}");
}
