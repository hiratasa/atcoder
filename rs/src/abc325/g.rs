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

fn solve0(s: &[char], k: usize, memo: &mut FxHashMap<Vec<char>, usize>) -> usize {
    if let Some(&r) = memo.get(s) {
        return r;
    }

    if s.is_empty() {
        return 0;
    }

    let n = s.len();
    let r = (0..n)
        .tuple_windows()
        .filter(|&(i, j)| s[i] == 'o' && s[j] == 'f')
        .filter_map(|(i, _)| {
            (i + 2..=min(i + 2 + k, n))
                .map(|l| {
                    let mut t = s[..i].to_vec();
                    t.extend(s[l..n].citer());

                    solve0(&t, k, memo)
                })
                .min()
        })
        .min()
        .unwrap_or(s.len());

    memo.insert(s.to_vec(), r);

    r
}

fn main() {
    input! {
        s: Chars,
        k: usize,
    };

    let n = s.len();
    let dp = (0..=n)
        .flat_map(|len| (0..=n - len).map(move |i| (i, i + len)))
        .fold(vec![vec![0usize; n + 1]; n + 1], |mut dp, (i, j)| {
            if i < j {
                dp[i][j] = (i + 1..j)
                    .map(|mid| dp[i][mid] + dp[mid][j])
                    .min()
                    .unwrap_or(j - i);

                if s[i] == 'o' {
                    dp[i][j] = min(
                        dp[i][j],
                        (i + 1..j)
                            .filter(|&l| s[l] == 'f' && dp[i + 1][l] == 0)
                            .map(|l| dp[l + 1][j].saturating_sub(k))
                            .min()
                            .unwrap_or(usize::MAX),
                    );
                }
            }

            dp
        });

    let ans = dp[0][n];

    println!("{ans}");
}
