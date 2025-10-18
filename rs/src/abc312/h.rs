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

#[allow(dead_code)]
fn z_algorithm<T: std::cmp::Eq>(s: &[T]) -> Vec<usize> {
    let n = s.len();

    // z[i] = max_{j<n} s[0:j] = s[i:i+j]
    let mut z = vec![0; n];
    z[0] = n;

    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
        // assert!(s[l..r] == s[0..r - l]);
        if i < r && z[i - l] < r - i {
            z[i] = z[i - l];
        } else {
            // i < rなら、 z[i - l] >= r - i なので、
            // s[i..r] (=s[i-l..r-l]) = s[0..r-i] が保証されている
            // i >= r なら再計算
            l = i;
            r = std::cmp::max(i, r);
            while r < n && s[r] == s[r - l] {
                r += 1;
            }
            z[i] = r - l;
        }
    }

    z
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let t = s
        .iter()
        .map(|word| {
            let z = z_algorithm(&word);

            let p = (1..word.len())
                .filter(|&i| word.len() % i == 0)
                .find(|&i| z[i] == word.len() - i)
                .unwrap_or(word.len());

            (word.len() / p, word[..p].to_vec())
        })
        .collect::<Vec<_>>();

    let groups = t
        .iter()
        .enumerate()
        .fold(FxHashMap::default(), |mut map, (i, (r, unit))| {
            map.entry(unit).or_insert(vec![]).push((i, *r));
            map
        });

    let ans = groups.values().fold(vec![0; n], |mut ans, v| {
        let s = v.citer().map(|(_, r)| r).sum::<usize>();

        v.citer()
            .scan(
                (vec![false; s + 1], vec![1; s + 1]),
                |(used, itrs), (i, r)| {
                    let idx = (itrs[r]..).find(|&idx| !used[r * idx]).unwrap();
                    used[r * idx] = true;
                    itrs[r] = idx + 1;

                    Some((i, idx))
                },
            )
            .for_each(|(i, x)| {
                ans[i] = x;
            });
        ans
    });

    println!("{}", ans.citer().join(" "));
}
