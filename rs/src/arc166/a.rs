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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom};
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
        cases: [(usize, Chars, Chars)]
    }

    cases
        .iter()
        .map(|(n, x, y)| {
            let check = |st: usize, ed: usize| {
                if st == ed {
                    return true;
                }

                let num_a0 = x[st..ed].citer().filter(|&c| c == 'A').count();
                let num_b0 = x[st..ed].citer().filter(|&c| c == 'B').count();
                let num_c0 = x[st..ed].citer().filter(|&c| c == 'C').count();
                let num_a1 = y[st..ed].citer().filter(|&c| c == 'A').count();
                let num_b1 = y[st..ed].citer().filter(|&c| c == 'B').count();
                let num_c1 = y[st..ed].citer().filter(|&c| c == 'C').count();
                assert!(num_c1 == 0);

                if num_a0 > num_a1 || num_b0 > num_b1 {
                    return false;
                }

                let c_to_a = num_a1 - num_a0;
                // let c_to_b = num_b1 - num_b0;

                izip!(x[st..ed].citer(), y[st..ed].citer())
                    .scan(0, |num_seen_c, (c, d)| {
                        if c == 'C' {
                            *num_seen_c += 1;
                            if *num_seen_c <= c_to_a {
                                Some(('A', d))
                            } else {
                                Some(('B', d))
                            }
                        } else {
                            Some((c, d))
                        }
                    })
                    .scan(0, |unmatched_b, (c, d)| {
                        if d == 'B' {
                            *unmatched_b += 1;
                        }
                        if c == 'B' {
                            if *unmatched_b > 0 {
                                *unmatched_b -= 1;
                                Some(true)
                            } else {
                                Some(false)
                            }
                        } else {
                            Some(true)
                        }
                    })
                    .all(|ok| ok)
            };

            let mut st = 0;
            for (i, (c, d)) in izip!(x.citer(), y.citer()).enumerate() {
                if d == 'C' {
                    if c != 'C' {
                        return false;
                    }
                    if !check(st, i) {
                        return false;
                    }
                    st = i + 1;
                }
            }
            if !check(st, *n) {
                return false;
            }

            true
        })
        .for_each(|ans| {
            if ans {
                println!("Yes");
            } else {
                println!("No");
            }
        })
}
