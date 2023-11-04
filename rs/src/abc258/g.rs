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

enum BitSet01 {}

impl Readable for BitSet01 {
    type Output = BitSet;
    fn read<R: std::io::BufRead, S: Source<R>>(source: &mut S) -> BitSet {
        let v = source
            .next_token_unwrap()
            .chars()
            .map(|c| c == '1')
            .collect::<Vec<_>>();

        v.citer()
            .enumerate()
            .fold(BitSet::new(v.len()), |mut bs, (i, x)| {
                bs.set(i, x);
                bs
            })
    }
}

fn main() {
    input! {
        n: usize,
        a: [BitSet01; n],
    }

    let ans = iproduct!(0..n, 0..n)
        .filter(|&(i, j)| a[i][j])
        .map(|(i, j)| (&a[i] & &a[j]).count_ones() as usize)
        .sum::<usize>()
        / 6;

    println!("{ans}");
}
