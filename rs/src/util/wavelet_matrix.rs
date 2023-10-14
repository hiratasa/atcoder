use itertools::Itertools;
use itertools_num::ItertoolsNum;
use std::iter::once;

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

type Value = u64;
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

    // 未実装
    //  * topk(): ある範囲で出現回数の大きい順にk個の値を返す
    //  * sum(): ある範囲の和
    //  * intersect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bv_rank() {
        let v = [false, false, true, false, false, true, false];
        let bv = BitVector::new(v);

        let mut rank0 = 0;
        let mut rank1 = 0;
        for i in 0..=v.len() {
            assert_eq!(rank0, bv.rank0(i));
            assert_eq!(rank1, bv.rank1(i));

            if i < v.len() {
                if v[i] {
                    rank1 += 1;
                } else {
                    rank0 += 1;
                }
            }
        }
    }

    #[test]
    fn test_bv_select() {
        let v = [false, false, true, false, false, true, false];
        let bv = BitVector::new(v);

        for i in 0..=bv.num0() {
            assert_eq!(v.iter().positions(|&x| !x).nth(i), bv.select0(i), "nth={i}");
        }

        for i in 0..=bv.num1() {
            assert_eq!(v.iter().positions(|&x| x).nth(i), bv.select1(i), "nth={i}");
        }
    }

    #[test]
    fn test_get() {
        let a = [1, 100, 34, 22, 9, 8, 77777, 6, 5, 34, 22, 9, 1, 4];
        let wm = WaveletMatrix::new(&a);

        for i in 0..a.len() {
            assert_eq!(a[i], wm.get(i));
        }
    }

    #[test]
    fn test_rank() {
        let a = [1, 100, 34, 22, 9, 8, 77777, 6, 5, 34, 22, 9, 1, 4];
        let wm = WaveletMatrix::new(&a);

        for i in 0..a.len() {
            for j in i + 1..a.len() {
                for value in a.iter().copied().chain(once(42)) {
                    let rank = a[i..j].iter().filter(|&&x| x == value).count();

                    assert_eq!(
                        rank,
                        wm.rank(i, j, value),
                        "start={i}, end={j}, value={value}"
                    );
                }
            }
        }
    }

    #[test]
    fn test_rank_below() {
        let a = [1, 100, 34, 22, 9, 8, 77777, 6, 5, 34, 22, 9, 1, 4];
        let wm = WaveletMatrix::new(&a);

        for i in 0..a.len() {
            for j in i + 1..a.len() {
                for value in a.iter().copied().flat_map(|x| [x, x + 1]).chain(once(42)) {
                    let rank = a[i..j].iter().filter(|&&x| x < value).count();

                    assert_eq!(
                        rank,
                        wm.rank_below(i, j, value),
                        "start={i}, end={j}, value={value}"
                    );
                }
            }
        }
    }

    #[test]
    fn test_select() {
        let a = [1, 100, 34, 22, 9, 8, 77777, 6, 5, 34, 22, 9, 1, 4];
        let wm = WaveletMatrix::new(&a);

        let mut freq = std::collections::BTreeMap::new();
        for i in 0..a.len() {
            *freq.entry(a[i]).or_insert(0) += 1;
            assert_eq!(
                Some(i),
                wm.select(freq[&a[i]] - 1, a[i]),
                "nth={}, value={}",
                freq[&a[i]] - 1,
                a[i]
            );
        }

        assert!(wm.select(0, 42).is_none());
        assert!(wm.select(1, 42).is_none());
    }

    #[test]
    fn test_quantile() {
        let a = [1, 100, 34, 22, 9, 8, 77777, 6, 5, 34, 22, 9, 1, 4];
        let wm = WaveletMatrix::new(&a);

        for i in 0..a.len() {
            for j in i + 1..a.len() {
                let mut v = a[i..j].to_vec();
                v.sort();

                for (idx, &val) in v.iter().enumerate() {
                    assert_eq!(val, wm.quantile(i, j, idx));
                }
            }
        }
    }
}
