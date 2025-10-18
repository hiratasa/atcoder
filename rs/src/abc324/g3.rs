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
struct BitVector {
    len: usize,
    blocks: Vec<u64>,
    ranks: Vec<usize>,
}

impl BitVector {
    const W: usize = 64;

    fn new(iter: impl IntoIterator<Item = bool>) -> BitVector {
        let mut len = 0;
        let blocks = iter
            .into_iter()
            .chunks(Self::W)
            .into_iter()
            .map(|it| {
                it.inspect(|_| {
                    len += 1;
                })
                .enumerate()
                .map(|(i, x)| if x { 1 << i } else { 0 })
                .sum::<u64>()
            })
            .collect::<Vec<_>>();

        let ranks = once(0)
            .chain(
                blocks
                    .iter()
                    .copied()
                    .map(|block| block.count_ones() as usize),
            )
            .cumsum::<usize>()
            .collect::<Vec<_>>();

        BitVector { len, blocks, ranks }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn get(&self, i: usize) -> bool {
        (self.blocks[i / Self::W] >> (i % Self::W)) & 1 > 0
    }

    // [0, i)の0の個数
    fn rank0(&self, i: usize) -> usize {
        i - self.rank1(i)
    }

    // [0, i)の1の個数
    fn rank1(&self, i: usize) -> usize {
        if i / Self::W >= self.blocks.len() {
            self.ranks[self.ranks.len() - 1]
        } else {
            self.ranks[i / Self::W]
                + (self.blocks[i / Self::W] & ((1 << (i % Self::W)) - 1)).count_ones() as usize
        }
    }

    fn num0(&self) -> usize {
        self.len - self.num1()
    }

    fn num1(&self) -> usize {
        self.ranks[self.ranks.len() - 1]
    }

    // nth番目(0-indexed)の0の位置
    fn select0(&self, nth: usize) -> Option<usize> {
        if nth >= self.num0() {
            return None;
        }

        let mut begin = 0;
        let mut end = self.ranks.len();
        while end - begin > 1 {
            let mid = begin + (end - begin) / 2;
            if nth < Self::W * mid - self.ranks[mid] {
                end = mid;
            } else {
                begin = mid;
            }
        }

        assert!(Self::W * begin - self.ranks[begin] <= nth);
        assert!(Self::W * (begin + 1) - self.ranks[begin + 1] > nth);

        let idx = begin;
        let pattern = self.blocks[idx];
        let nth_in_block = nth - self.ranks[idx];
        let mut begin = 0;
        let mut end = Self::W;
        while end - begin > 1 {
            let mid = begin + (end - begin) / 2;
            if nth_in_block < mid - (pattern & ((1 << mid) - 1)).count_ones() as usize {
                end = mid;
            } else {
                begin = mid;
            }
        }

        assert!(pattern & (1 << begin) == 0);

        Some(idx * Self::W + begin)
    }

    // nth番目(0-indexed)の1の位置
    fn select1(&self, nth: usize) -> Option<usize> {
        if nth >= self.num1() {
            return None;
        }

        let mut begin = 0;
        let mut end = self.ranks.len();
        while end - begin > 1 {
            let mid = begin + (end - begin) / 2;
            if nth < self.ranks[mid] {
                end = mid;
            } else {
                begin = mid;
            }
        }

        assert!(self.ranks[begin] <= nth);
        assert!(self.ranks[begin + 1] > nth);

        let idx = begin;
        let pattern = self.blocks[idx];
        let nth_in_block = nth - self.ranks[idx];
        let mut begin = 0;
        let mut end = Self::W;
        while end - begin > 1 {
            let mid = begin + (end - begin) / 2;
            if nth_in_block < (pattern & ((1 << mid) - 1)).count_ones() as usize {
                end = mid;
            } else {
                begin = mid;
            }
        }

        assert!(pattern & (1 << begin) > 0);

        Some(idx * Self::W + begin)
    }
}

// 参考: https://miti-7.hatenablog.com/entry/2018/04/28/152259
struct WaveletMatrix {
    bits: Vec<BitVector>,
}

type Value = usize;
#[allow(dead_code)]
impl WaveletMatrix {
    const W: usize = 64;

    fn new(values: &[Value]) -> WaveletMatrix {
        let n = values.len();

        let bits = (0..Self::W)
            .rev()
            .scan(values.to_vec(), |values, i| {
                let bv = BitVector::new(values.iter().map(|&x| x & (1 << i) > 0));

                let mut new_values = vec![0; n];
                values
                    .iter()
                    .filter(|&x| x & (1 << i) == 0)
                    .zip(new_values.iter_mut())
                    .for_each(|(val, to)| {
                        *to = *val;
                    });
                values
                    .iter()
                    .filter(|&x| x & (1 << i) > 0)
                    .zip(new_values[bv.num0()..].iter_mut())
                    .for_each(|(val, to)| {
                        *to = *val;
                    });
                *values = new_values;

                Some(bv)
            })
            .collect::<Vec<_>>();

        WaveletMatrix { bits }
    }

    // i番目の要素
    // O(W)
    fn get(&self, i: usize) -> Value {
        self.bits
            .iter()
            .fold((0, i), |(val, idx), bv| {
                let b = bv.get(idx);

                if !b {
                    (2 * val, bv.rank0(idx))
                } else {
                    (2 * val + 1, bv.num0() + bv.rank1(idx))
                }
            })
            .0
    }

    // [0, idx)での最後のvalの一番最後のbit vectorでの位置+1 (内部計算用)
    // idxに0を渡した場合は一番最後のbit vectorでのvalの最初の出現位置
    // O(W)
    fn value_idx(&self, idx: usize, val: Value) -> usize {
        self.bits.iter().enumerate().fold(idx, |idx, (i, bv)| {
            if val & (1 << (Self::W - 1 - i)) == 0 {
                bv.rank0(idx)
            } else {
                bv.num0() + bv.rank1(idx)
            }
        })
    }

    // [begin, end)でのvalの出現回数
    // O(W)
    fn rank(&self, begin: usize, end: usize, val: Value) -> usize {
        let begin_idx = self.value_idx(begin, val);
        let end_idx = self.value_idx(end, val);

        end_idx - begin_idx
    }

    // [begin, end)でのval未満の値の出現回数
    // O(W)
    fn rank_below(&self, begin: usize, end: usize, val: Value) -> usize {
        self.bits
            .iter()
            .enumerate()
            .fold((0, begin, end), |(num, begin, end), (i, bv)| {
                if val & (1 << (Self::W - 1 - i)) == 0 {
                    (num, bv.rank0(begin), bv.rank0(end))
                } else {
                    (
                        num + bv.rank0(end) - bv.rank0(begin),
                        bv.num0() + bv.rank1(begin),
                        bv.num0() + bv.rank1(end),
                    )
                }
            })
            .0
    }

    // [begin, end)での[lower,upper)の値の出現回数
    // O(W)
    fn rank_between(&self, begin: usize, end: usize, lower: Value, upper: Value) -> usize {
        self.rank_below(begin, end, upper) - self.rank_below(begin, end, lower)
    }

    // nth番目のvalの出現位置
    // O(W * log(n))
    fn select(&self, nth: usize, val: Value) -> Option<usize> {
        let begin_idx = self.value_idx(0, val);

        let idx = begin_idx + nth;
        if idx >= self.bits[0].len() {
            return None;
        }

        self.bits
            .iter()
            .rev()
            .enumerate()
            .try_fold(idx, |idx, (i, bv)| {
                let idx = if val & (1 << i) == 0 {
                    bv.select0(idx)?
                } else {
                    bv.select1(idx.checked_sub(bv.num0())?)?
                };

                let b = bv.get(idx);

                if (val & (1 << i) > 0) != b {
                    return None;
                }

                Some(idx)
            })
    }

    // [begin, end)でのr番目(0-indexed)に小さい値
    // O(W)
    fn quantile(&self, begin: usize, end: usize, r: usize) -> Value {
        assert!(r < end - begin);

        self.bits
            .iter()
            .fold((0, begin, end, r), |(val, begin, end, r), bv| {
                let num1 = bv.rank1(end) - bv.rank1(begin);
                let num0 = (end - begin) - num1;

                if r < num0 {
                    (val * 2, bv.rank0(begin), bv.rank0(end), r)
                } else {
                    (
                        val * 2 + 1,
                        bv.num0() + bv.rank1(begin),
                        bv.num0() + bv.rank1(end),
                        r - num0,
                    )
                }
            })
            .0
    }
}

#[allow(dead_code)]
fn lower_bound<T, F>(mut begin: T, mut end: T, epsilon: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    let two = T::try_from(2).ok().unwrap();
    while end - begin >= epsilon {
        let mid = begin + (end - begin) / two;
        match f(mid) {
            std::cmp::Ordering::Less => {
                begin = mid + epsilon;
            }
            _ => {
                end = mid;
            }
        }
    }
    begin
}
#[allow(dead_code)]
fn lower_bound_int<T, F>(begin: T, end: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    lower_bound(begin, end, T::try_from(1).ok().unwrap(), f)
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        query: [(usize, usize, usize)],
    }

    let b = (0..n).sorted_by_key(|&i| a[i]).collect::<Vec<_>>();
    let start_idxs = (0..=n + 1)
        .scan(0, |idx, i| {
            while *idx < n && a[b[*idx]] < i {
                *idx += 1;
            }

            Some(*idx)
        })
        .collect::<Vec<_>>();

    let wm = WaveletMatrix::new(&a);
    let wm2 = WaveletMatrix::new(&b);

    query
        .citer()
        .scan(vec![(0, n, 1, n + 1)], |table, (t, s, x)| {
            if t == 1 {
                let (start, end, lower, upper) = table[s];

                let lower_idx = start_idxs[lower];
                let upper_idx = start_idxs[upper];

                let y = wm2.rank_below(lower_idx, upper_idx, start) + x;

                let mid = if y >= upper_idx - lower_idx {
                    end
                } else {
                    wm2.quantile(lower_idx, upper_idx, y)
                };
                let mid = min(mid, end);

                table[s] = (start, mid, lower, upper);
                table.push((mid, end, lower, upper));

                Some(wm.rank_between(mid, end, lower, upper))
            } else {
                let (start, end, lower, upper) = table[s];

                let xx = max(lower, min(upper, x + 1));
                table[s] = (start, end, lower, xx);
                table.push((start, end, xx, upper));

                Some(wm.rank_between(start, end, xx, upper))
            }
        })
        .for_each(|ans| {
            println!("{ans}");
        });
}
