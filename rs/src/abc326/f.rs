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
        n: usize, x: i64, y: i64,
        a: [i64; n]
    }

    let ans = izip!(0..2, [y, x])
        .map(|(p, z)| {
            let b = a.citer().skip(p).step_by(2).collect::<Vec<_>>();

            let m = b.len();
            let k0 = m / 2;
            let k1 = m - k0;

            let set0 = (0..1 << k0)
                .map(|s| {
                    (0..k0)
                        .map(|i| if s & (1 << i) > 0 { b[i] } else { -b[i] })
                        .sum::<i64>()
                })
                .collect::<Vec<_>>();
            let set1 = (0..1 << k1)
                .map(|s| {
                    (0..k1)
                        .map(|i| {
                            if s & (1 << i) > 0 {
                                b[k0 + i]
                            } else {
                                -b[k0 + i]
                            }
                        })
                        .sum::<i64>()
                })
                .collect::<FxHashSet<_>>();

            if let Some(t0) = set0.citer().find(|&t| set1.contains(&(z - t))) {
                let t1 = z - t0;

                let mask0 = (0..1 << k0)
                    .find(|s| {
                        (0..k0)
                            .map(|i| if s & (1 << i) > 0 { b[i] } else { -b[i] })
                            .sum::<i64>()
                            == t0
                    })
                    .unwrap();
                let mask1 = (0..1 << k1)
                    .find(|s| {
                        (0..k1)
                            .map(|i| {
                                if s & (1 << i) > 0 {
                                    b[k0 + i]
                                } else {
                                    -b[k0 + i]
                                }
                            })
                            .sum::<i64>()
                            == t1
                    })
                    .unwrap();

                Some(
                    (0..k0)
                        .map(|i| mask0 & (1 << i) > 0)
                        .chain((0..k1).map(|i| mask1 & (1 << i) > 0))
                        .collect::<Vec<_>>(),
                )
            } else {
                None
            }
        })
        .next_tuple()
        .map(|(ans0, ans1)| {
            ans0.and_then(|ans0| {
                ans1.map(|ans1| {
                    let mut ans = vec!['\0'; n];

                    ans[0] = if ans0[0] { 'L' } else { 'R' };
                    for i in (0..n).step_by(2).skip(1) {
                        let c = match (ans1[i / 2 - 1], ans0[i / 2]) {
                            (false, false) => 'L', // 左→下
                            (false, true) => 'R',  // 左→上
                            (true, false) => 'R',  // 右→下
                            (true, true) => 'L',   // 右→上
                        };
                        ans[i] = c;
                    }
                    for i in (0..n).skip(1).step_by(2) {
                        let c = match (ans0[(i - 1) / 2], ans1[(i - 1) / 2]) {
                            (false, false) => 'R', // 下→左
                            (false, true) => 'L',  // 下→右
                            (true, false) => 'L',  // 上→左
                            (true, true) => 'R',   // 上→右
                        };
                        ans[i] = c;
                    }

                    ans
                })
            })
        })
        .unwrap();

    if let Some(ans) = ans {
        println!("Yes");
        println!("{}", ans.citer().join(""));
    } else {
        println!("No");
    }
}
