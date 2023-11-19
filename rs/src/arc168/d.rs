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

fn main() {
    input! {
        n: usize, m: usize,
        mut lr: [(Usize1, usize); m],
    };

    let mut exists1 = vec![vec![false; n + 1]; n + 1];
    let mut exists = vec![vec![false; n + 1]; n + 1];
    for (l, r) in lr.citer() {
        if l + 1 == r {
            exists1[l][l] = true;
        }
        exists[l][r] = true;
    }

    let dp = (1..=n)
        .flat_map(|len| (0..=n - len).map(move |i| (i, i + len)))
        .fold(vec![vec![0; n + 1]; n + 1], |mut dp, (i, j)| {
            dp[i][j] = (i..j)
                .filter_map(|k| {
                    exists1[i][k] = exists1[i][k] || exists1[i + 1][k] || exists[i][j];
                    if exists1[i][k] {
                        Some(1 + dp[i][k] + dp[k + 1][j])
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or(0);

            dp
        });

    let ans = dp[0][n];

    println!("{ans}");
}
