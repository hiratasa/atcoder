#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64;
#[allow(unused_imports)]
use std::i64;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::iter::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom, Rng, SeedableRng};
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;

#[allow(unused_imports)]
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
#[allow(unused_imports)]
use proconio::source::{Readable, Source};

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
macro_rules! it {
    ($x:expr) => {
        once($x)
    };
    ($first:expr,$($x:expr),+) => {
        chain(
            once($first),
            it!($($x),+)
        )
    };
    ($($x:expr),+,) => {
        it![$($x),+]
    };
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

#[allow(unused_macros)]
macro_rules! pushed {
    ($c:expr, $x:expr) => {{
        let x = $x;
        let mut c = $c;
        c.push(x);
        c
    }};
}

#[allow(unused_macros)]
macro_rules! inserted {
    ($c:expr, $($x:expr),*) => {{
        let mut c = $c;
        c.insert($($x),*);
        c
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
        n: usize, h: usize,
        x: [usize; n],
        pf: [(usize, usize); n - 1]
    }

    const M: usize = 300;
    let mut init = vec![vec![None; M + 1]; M + 1];
    init[h][0] = Some(Reverse(0));

    let (last_x, dp) =
        izip!(x.citer(), pf.citer()).fold((0, init), |(prev_pos, prev), (pos, (p, f))| {
            let mut next = vec![vec![None; M + 1]; M + 1];

            for i in 0..=M {
                for j in 0..=M {
                    if let Some(Reverse(c)) = prev[i][j] {
                        if i < pos - prev_pos {
                            continue;
                        }
                        if j + (pos - prev_pos) > h {
                            continue;
                        }

                        // 使わない
                        {
                            let ii = i - (pos - prev_pos);
                            let jj = j + (pos - prev_pos);
                            next[ii][jj] = max(next[ii][jj], Some(Reverse(c)));
                        }

                        // 往路で使う
                        {
                            let ii = min(h, i - (pos - prev_pos) + f);
                            let jj = j + (pos - prev_pos);
                            next[ii][jj] = max(next[ii][jj], Some(Reverse(c + p)));
                        }

                        // 復路で使う
                        {
                            let ii = i - (pos - prev_pos);
                            let jj = (j + (pos - prev_pos)).saturating_sub(f);
                            next[ii][jj] = max(next[ii][jj], Some(Reverse(c + p)));
                        }
                    }
                }
            }

            (pos, next)
        });

    let r = x[n - 1] - last_x;
    let ans = iproduct!(0..=M, 0..=M)
        .filter_map(|(i, j)| {
            let Reverse(c) = dp[i][j]?;

            if i < r {
                return None;
            }

            if i - r < j + r {
                return None;
            }

            Some(c)
        })
        .min();

    println_opt(ans);
}
