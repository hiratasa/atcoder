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
        n: usize, q: usize,
        c: [usize; n],
        query: [(Usize1, Usize1); q],
    };

    let merge = |from: FxHashSet<usize>, to: &mut FxHashSet<usize>| {
        let (smaller, mut larger) = if from.len() < to.len() {
            (from, take(to))
        } else {
            (take(to), from)
        };

        larger.extend(smaller);

        *to = larger;
    };

    query
        .citer()
        .scan(
            c.citer()
                .map(|x| once(x).collect::<FxHashSet<_>>())
                .collect::<Vec<_>>(),
            |sets, (a, b)| {
                let aa = take(&mut sets[a]);

                merge(aa, &mut sets[b]);

                Some(sets[b].len())
            },
        )
        .for_each(|ans| {
            println!("{ans}");
        })
}
