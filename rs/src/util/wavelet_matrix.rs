use std::iter::once;

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
                if let Some(x) = x_opt {
                    len += 1;
                    if x {
                        *b += 1 << (i % W);
                    }

                    if i % W == W - 1 {
                        Some(Some(std::mem::replace(b, 0)))
                    } else {
                        Some(None)
                    }
                } else {
                    Some(Some(*b))
                }
            })
            .flatten()
            .collect::<Vec<_>>();

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

type Value = u64;
#[allow(dead_code)]
impl WaveletMatrix {
    const H: usize = 64;

    fn new(values: &[Value]) -> WaveletMatrix {
        let n = values.len();

        let bits = (0..Self::H)
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
            if val & (1 << (Self::H - 1 - i)) == 0 {
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
        let mut num = 0;
        let mut begin = begin;
        let mut end = end;

        for i in 0..Self::H {
            let bv = &self.bits[i];
            let pos = 1 << (Self::H - 1 - i);

            let add_num;
            (add_num, begin, end) = if val & pos == 0 {
                (0, bv.count0(begin), bv.count0(end))
            } else {
                let c1b = bv.count1(begin);
                let c1e = bv.count1(end);
                (
                    (end - begin) - (c1e - c1b),
                    bv.num0() + c1b,
                    bv.num0() + c1e,
                )
            };

            num += add_num;
        }

        num
    }

    // [begin, end)での[lower, upper)の出現回数
    // O(W)
    fn count_between(&self, begin: usize, end: usize, lower: Value, upper: Value) -> usize {
        let mut num = 0;
        let mut begin_l = begin;
        let mut end_l = end;
        let mut begin_u = begin;
        let mut end_u = end;

        for i in 0..Self::H {
            let bv = &self.bits[i];
            let pos = 1 << (Self::H - 1 - i);

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

    // 未実装
    //  * topk(): ある範囲で出現回数の大きい順にk個の値を返す
    //  * sum(): ある範囲の和
    //  * intersect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_bv_rank() {
        let v = [false, false, true, false, false, true, false];
        let bv = BitVector::new(v);

        let mut rank0 = 0;
        let mut rank1 = 0;
        for i in 0..=v.len() {
            assert_eq!(rank0, bv.count0(i));
            assert_eq!(rank1, bv.count1(i));

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
                        wm.count(i, j, value),
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
                        wm.count_below(i, j, value),
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
