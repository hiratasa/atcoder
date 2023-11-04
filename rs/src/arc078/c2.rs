#[allow(unused_imports)]
use std::{cmp::*, collections::*, f64, i64, io, iter::*, mem::*, str::*, usize};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
use proconio::input_interactive;
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

fn query(n: usize) -> bool {
    assert!(n > 0);
    assert!(n <= 1000000000000000000);

    println!("? {}", n);

    input_interactive! {
        c: char,
    }

    c == 'Y'
}

fn solve() -> usize {
    if !query(999999999) {
        return 1000000000;
    }

    let mut low = 0;
    let mut high;
    for _ in 0..9 {
        (low, high) = (low * 10, low * 10 + 9);

        while low < high {
            let mid = (low + high + 1) / 2;
            assert!(mid > 0);

            if query(mid * 1000000000) {
                high = mid - 1;
            } else {
                low = mid;
            }
        }
    }

    low += 1;
    while low % 10 == 0 {
        low /= 10;
    }

    let mut x = low;
    loop {
        let q = query(x + 1);

        let s0 = format!("{}", x);
        let s = format!("{}", x + 1);

        if (s > s0) == q {
            return x;
        }

        x *= 10;
    }
}

fn main() {
    println!("! {}", solve());
}
