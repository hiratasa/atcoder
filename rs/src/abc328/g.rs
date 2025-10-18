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
        n: usize, c: usize,
        a: [usize; n],
        b: [usize; n],
    };

    let d = iproduct!(0..n, 0..n, 0..=n)
        .filter(|&(i, j, l)| i + l <= n && j + l <= n)
        .fold(
            vec![vec![vec![usize::MAX; n + 1]; n]; n],
            |mut d, (i, j, l)| {
                d[i][j][l] = izip!(a[i..i + l].citer(), b[j..j + l].citer())
                    .map(|(x, y)| x.abs_diff(y))
                    .sum::<usize>();
                d
            },
        );

    let dp = (1usize..1 << n)
        .rev()
        .fold(vec![usize::MAX; 1 << n], |mut dp, s| {
            if s == (1 << n) - 1 {
                dp[s] = 0;
            }

            let k = n - s.count_ones() as usize;

            let mut st = 0usize;
            while s >> st != 0 {
                st += (s >> st).trailing_zeros() as usize;

                let max_l = (s >> st).trailing_ones() as usize;
                for l in 1..=max_l {
                    let p = (1 << (st + l)) - (1 << st);
                    assert!(s & p == p);

                    dp[s ^ p] = min(dp[s ^ p], dp[s] + c + d[st][k][l]);
                }

                st += 1;
            }

            dp
        });

    let ans = dp[0] - c;

    println!("{ans}");
}
