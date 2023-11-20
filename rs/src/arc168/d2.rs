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

type BitBlock = u16;
const W: usize = std::mem::size_of::<BitBlock>() * 8;

const fn construct_masks() -> [BitBlock; W] {
    let mut v = [0; W];

    let mut i = 0;
    while i < W {
        v[i] = (1 << i) - 1;
        i += 1;
    }

    v
}

const fn construct_num_ones() -> [usize; 1 << W] {
    let mut v = [0; 1 << W];

    let mut i = 0;
    while i < 1 << W {
        v[i] = i.count_ones() as usize;
        i += 1;
    }

    v
}

const MASKS: [BitBlock; W] = construct_masks();
const NUM_ONES: [usize; 1 << W] = construct_num_ones();

struct BitVector {
    len: usize,
    blocks: Vec<BitBlock>,
    ranks: Vec<usize>,
}

impl BitVector {
    const W: usize = W;

    fn new(iter: impl IntoIterator<Item = bool>) -> BitVector {
        let mut len = 0;
        let blocks = iter
            .into_iter()
            .map(|x| Some(x))
            .chain(once(None)) // count1()などにlenぴったりが渡されても問題ないように.
            .enumerate()
            .scan(0 as BitBlock, |b, (i, x_opt)| {
                len += 1;

                if let Some(x) = x_opt {
                    if x {
                        *b += 1 << (i % W);
                    }

                    if i % W == W - 1 {
                        Some(Some(replace(b, 0)))
                    } else {
                        Some(None)
                    }
                } else {
                    Some(Some(*b))
                }
            })
            .flatten()
            .collect::<Vec<_>>();
        len -= 1;

        let ranks = once(0)
            .chain(
                blocks
                    .iter()
                    .copied()
                    .map(|block| block.count_ones() as usize),
            )
            .scan(0, |s, x| {
                *s += x;
                Some(*s)
            })
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
    fn count0(&self, i: usize) -> usize {
        i - self.count1(i)
    }

    // [0, i)の1の個数
    fn count1(&self, i: usize) -> usize {
        self.ranks[i / Self::W] + NUM_ONES[(self.blocks[i / Self::W] & MASKS[i % Self::W]) as usize]
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
        let nth_in_block = nth - (Self::W * begin - self.ranks[idx]);
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
    const W: usize = 9;

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
                    (2 * val, bv.count0(idx))
                } else {
                    (2 * val + 1, bv.num0() + bv.count1(idx))
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
                bv.count0(idx)
            } else {
                bv.num0() + bv.count1(idx)
            }
        })
    }

    // [begin, end)でのvalの出現回数
    // O(W)
    fn count(&self, begin: usize, end: usize, val: Value) -> usize {
        let begin_idx = self.value_idx(begin, val);
        let end_idx = self.value_idx(end, val);

        end_idx - begin_idx
    }

    // [begin, end)でのval未満の値の出現回数
    // O(W)
    fn count_below(&self, begin: usize, end: usize, val: Value) -> usize {
        self.bits
            .iter()
            .enumerate()
            .fold((0, begin, end), |(num, begin, end), (i, bv)| {
                if val & (1 << (Self::W - 1 - i)) == 0 {
                    (num, bv.count0(begin), bv.count0(end))
                } else {
                    let c1b = bv.count1(begin);
                    let c1e = bv.count1(end);
                    (
                        num + (end - begin) - (c1e - c1b),
                        bv.num0() + c1b,
                        bv.num0() + c1e,
                    )
                }
            })
            .0
    }

    // [begin, end)での[lower, upper)の出現回数
    // O(W)
    fn count_between(&self, begin: usize, end: usize, lower: Value, upper: Value) -> usize {
        let mut num = 0;
        let mut begin_l = begin;
        let mut end_l = end;
        let mut begin_u = begin;
        let mut end_u = end;

        for i in 0..Self::W {
            let bv = &self.bits[i];
            let pos = 1 << (Self::W - 1 - i);

            let num_l;
            (num_l, begin_l, end_l) = if lower & pos == 0 {
                (0, bv.count0(begin_l), bv.count0(end_l))
            } else {
                let c1b = bv.count1(begin_l);
                let c1e = bv.count1(end_l);
                (
                    (end_l - begin_l) - (c1e - c1b),
                    bv.num0() + c1b,
                    bv.num0() + c1e,
                )
            };

            let num_u;
            (num_u, begin_u, end_u) = if upper & pos == 0 {
                (0, bv.count0(begin_u), bv.count0(end_u))
            } else {
                let c1b = bv.count1(begin_u);
                let c1e = bv.count1(end_u);
                (
                    (end_u - begin_u) - (c1e - c1b),
                    bv.num0() + c1b,
                    bv.num0() + c1e,
                )
            };

            num = num + num_u - num_l;
        }

        num
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

                assert_eq!((val & (1 << i) > 0), bv.get(idx));

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
                let num1 = bv.count1(end) - bv.count1(begin);
                let num0 = (end - begin) - num1;

                if r < num0 {
                    (val * 2, bv.count0(begin), bv.count0(end), r)
                } else {
                    (
                        val * 2 + 1,
                        bv.num0() + bv.count1(begin),
                        bv.num0() + bv.count1(end),
                        r - num0,
                    )
                }
            })
            .0
    }
}

fn main() {
    input! {
       n: usize, m: usize,
       mut lr: [(Usize1, usize); m],
    };

    lr.sort();

    let rs = lr.citer().map(|(_, r)| r).collect::<Vec<_>>();

    let (mut starts, last) =
        lr.citer()
            .enumerate()
            .fold((vec![0; n + 1], 0), |(mut starts, last), (i, (l, _))| {
                for ll in last..l + 1 {
                    starts[ll] = i;
                }

                (starts, l + 1)
            });
    for ll in last..=n {
        starts[ll] = m;
    }

    let wv = WaveletMatrix::new(&rs);

    let dp = (1..=n)
        .flat_map(|len| (0..=n - len).map(move |i| (i, i + len)))
        .fold(vec![vec![0; n + 1]; n + 1], |mut dp, (i, j)| {
            dp[i][j] = (i..j)
                .filter_map(|k| {
                    let i0 = starts[i];
                    let i1 = starts[k + 1];

                    if wv.count_between(i0, i1, k + 1, j + 1) > 0 {
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
