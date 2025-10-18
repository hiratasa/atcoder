use std::iter::once;

trait BitVector {
    fn new(iter: impl IntoIterator<Item = bool>) -> Self;

    fn len(&self) -> usize;
    fn get(&self, i: usize) -> bool;
    // [0, i)の0の個数
    fn count0(&self, i: usize) -> usize;
    // [0, i)の1の個数
    fn count1(&self, i: usize) -> usize;
    // 全体の0の個数
    fn num0(&self) -> usize {
        self.count0(self.len())
    }
    // 全体の1の個数
    fn num1(&self) -> usize {
        self.count1(self.len())
    }
    // nth番目(0-indexed)の0の位置
    fn select0(&self, nth: usize) -> Option<usize>;
    // nth番目(0-indexed)の1の位置
    fn select1(&self, nth: usize) -> Option<usize>;
}

// サイズが小さく、キャッシュミスが問題にならないとき => BitVectorByRanks
// サイズが大きく、キャッシュミスが問題になるとき => BitSetVector

type BitRank = u8;

struct BitVectorByRanks {
    len: usize,
    ranks: Vec<BitRank>,
    group_ranks: Vec<usize>,
}

impl BitVectorByRanks {
    const GROUP_WIDTH: usize = BitRank::MAX as usize + 1;
}

impl BitVector for BitVectorByRanks {
    fn new(iter: impl IntoIterator<Item = bool>) -> BitVectorByRanks {
        let bits = iter.into_iter().collect::<Vec<_>>();
        let len = bits.len();

        let mut ranks = once(0)
            .chain(bits.iter().copied().map(|x| x as BitRank))
            .collect::<Vec<_>>();

        for i in 1..ranks.len() {
            if i % Self::GROUP_WIDTH == 0 {
                ranks[i] = 0;
            } else {
                ranks[i] += ranks[i - 1];
            }
        }

        let mut group_ranks = vec![0; (len + Self::GROUP_WIDTH - 1) / Self::GROUP_WIDTH + 1];
        for (i, b) in bits.iter().copied().enumerate() {
            group_ranks[(i / Self::GROUP_WIDTH) + 1] += b as usize;
        }

        for i in 1..group_ranks.len() {
            group_ranks[i] += group_ranks[i - 1];
        }

        BitVectorByRanks {
            len,
            ranks,
            group_ranks,
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn get(&self, i: usize) -> bool {
        self.count1(i + 1) > self.count1(i)
    }

    // [0, i)の0の個数
    fn count0(&self, i: usize) -> usize {
        i - self.count1(i)
    }

    // [0, i)の1の個数
    fn count1(&self, i: usize) -> usize {
        self.group_ranks[i / Self::GROUP_WIDTH] + self.ranks[i] as usize
    }

    fn num0(&self) -> usize {
        self.len - self.num1()
    }

    fn num1(&self) -> usize {
        self.group_ranks[self.group_ranks.len() - 1]
    }

    // nth番目(0-indexed)の0の位置
    fn select0(&self, nth: usize) -> Option<usize> {
        if nth >= self.num0() {
            return None;
        }

        let mut begin = 0;
        let mut end = self.len;
        while end - begin > 1 {
            let mid = begin + (end - begin) / 2;
            if nth < self.count0(mid) {
                end = mid;
            } else {
                begin = mid;
            }
        }

        assert!(self.count0(begin) <= nth);
        assert!(begin + 1 == self.len || self.count0(begin + 1) > nth);

        Some(begin)
    }

    // nth番目(0-indexed)の1の位置
    fn select1(&self, nth: usize) -> Option<usize> {
        if nth >= self.num1() {
            return None;
        }

        let mut begin = 0;
        let mut end = self.len;
        while end - begin > 1 {
            let mid = begin + (end - begin) / 2;
            if nth < self.count1(mid) {
                end = mid;
            } else {
                begin = mid;
            }
        }

        assert!(self.count1(begin) <= nth);
        assert!(begin + 1 == self.len || self.count1(begin + 1) > nth);

        Some(begin)
    }
}

type Block = u8;
type BlockRank = u8;

struct BitSetVector {
    len: usize,
    blocks: Vec<Block>,
    block_ranks: Vec<BlockRank>,
    block_group_ranks: Vec<usize>,
}

impl BitSetVector {
    const WIDTH: usize = std::mem::size_of::<Block>() * 8;
    const GROUP_WIDTH: usize = BlockRank::MAX as usize + 1;
    const BLOCK_PER_GROUP: usize = Self::GROUP_WIDTH / Self::WIDTH;

    const NUM_ONES: [usize; Self::WIDTH << Self::WIDTH] = Self::construct_num_ones();

    const fn construct_num_ones() -> [usize; Self::WIDTH << Self::WIDTH] {
        let mut v = [0; Self::WIDTH << Self::WIDTH];

        let mut i = 0;
        while i < 1 << Self::WIDTH {
            let mut j = 0;
            while j < Self::WIDTH {
                v[i * Self::WIDTH + j] = (i & ((1 << j) - 1)).count_ones() as usize;
                j += 1;
            }
            i += 1;
        }

        v
    }
}

impl BitVector for BitSetVector {
    fn new(iter: impl IntoIterator<Item = bool>) -> BitSetVector {
        let mut len = 0;
        let mut blocks = vec![];

        // count0(len), count1(len)で範囲外アクセスが起きないように、最後にダミー要素を足しておく
        for (i, x) in iter.into_iter().chain(once(false)).enumerate() {
            len += 1;
            if i / Self::WIDTH >= blocks.len() {
                blocks.push(0 as Block);
            }

            if x {
                blocks[i / Self::WIDTH] |= 1 << (i % Self::WIDTH);
            }
        }

        // ダミー要素の分
        len -= 1;

        let mut block_ranks = once(0)
            .chain(blocks.iter().map(|&b| b.count_ones() as BlockRank))
            .collect::<Vec<_>>();
        for i in 1..block_ranks.len() {
            if i % Self::BLOCK_PER_GROUP == 0 {
                block_ranks[i] = 0;
            } else {
                block_ranks[i] += block_ranks[i - 1];
            }
        }

        let mut block_group_ranks =
            vec![0; (len + 1 + Self::GROUP_WIDTH - 1) / Self::GROUP_WIDTH + 1];
        for (i, &b) in blocks.iter().enumerate() {
            block_group_ranks[(i / Self::BLOCK_PER_GROUP) + 1] += b.count_ones() as usize;
        }

        for i in 1..block_group_ranks.len() {
            block_group_ranks[i] += block_group_ranks[i - 1];
        }

        BitSetVector {
            len,
            blocks,
            block_ranks,
            block_group_ranks,
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn get(&self, i: usize) -> bool {
        (self.blocks[i / Self::WIDTH] >> (i % Self::WIDTH)) & 1 > 0
    }

    // [0, i)の0の個数
    fn count0(&self, i: usize) -> usize {
        i - self.count1(i)
    }

    // [0, i)の1の個数
    fn count1(&self, i: usize) -> usize {
        self.block_group_ranks[i / Self::GROUP_WIDTH]
            + self.block_ranks[i / Self::WIDTH] as usize
            + Self::NUM_ONES
                [self.blocks[i / Self::WIDTH] as usize * Self::WIDTH + (i % Self::WIDTH)]
    }

    fn num0(&self) -> usize {
        self.len - self.num1()
    }

    fn num1(&self) -> usize {
        self.block_group_ranks[self.block_group_ranks.len() - 1]
    }

    // nth番目(0-indexed)の0の位置
    fn select0(&self, nth: usize) -> Option<usize> {
        if nth >= self.num0() {
            return None;
        }

        let mut begin = 0;
        let mut end = self.blocks.len();
        while end - begin > 1 {
            let mid = begin + (end - begin) / 2;
            if nth
                < Self::WIDTH * mid
                    - (self.block_group_ranks[mid / Self::BLOCK_PER_GROUP]
                        + self.block_ranks[mid] as usize)
            {
                end = mid;
            } else {
                begin = mid;
            }
        }

        assert!(
            Self::WIDTH * begin
                - (self.block_group_ranks[begin / Self::BLOCK_PER_GROUP]
                    + self.block_ranks[begin] as usize)
                <= nth
        );
        assert!(
            begin + 1 == self.blocks.len()
                || Self::WIDTH * (begin + 1)
                    - (self.block_group_ranks[(begin + 1) / Self::BLOCK_PER_GROUP]
                        + self.block_ranks[begin + 1] as usize)
                    > nth
        );

        let idx = begin;
        let pattern = self.blocks[idx];
        let nth_in_block = nth
            - (Self::WIDTH * begin
                - (self.block_group_ranks[begin / Self::BLOCK_PER_GROUP]
                    + self.block_ranks[begin] as usize));
        let mut begin = 0;
        let mut end = Self::WIDTH;
        while end - begin > 1 {
            let mid = begin + (end - begin) / 2;
            if nth_in_block < mid - Self::NUM_ONES[pattern as usize * Self::WIDTH + mid] {
                end = mid;
            } else {
                begin = mid;
            }
        }

        assert!(pattern & (1 << begin) == 0);

        Some(idx * Self::WIDTH + begin)
    }

    // nth番目(0-indexed)の1の位置
    fn select1(&self, nth: usize) -> Option<usize> {
        if nth >= self.num1() {
            return None;
        }

        let mut begin = 0;
        let mut end = self.blocks.len();
        while end - begin > 1 {
            let mid = begin + (end - begin) / 2;
            if nth
                < self.block_group_ranks[mid / Self::BLOCK_PER_GROUP]
                    + self.block_ranks[mid] as usize
            {
                end = mid;
            } else {
                begin = mid;
            }
        }

        assert!(
            self.block_group_ranks[begin / Self::BLOCK_PER_GROUP]
                + self.block_ranks[begin] as usize
                <= nth
        );
        assert!(
            begin + 1 == self.blocks.len()
                || self.block_group_ranks[(begin + 1) / Self::BLOCK_PER_GROUP]
                    + self.block_ranks[begin + 1] as usize
                    > nth
        );

        let idx = begin;
        let pattern = self.blocks[idx];
        let nth_in_block = nth
            - (self.block_group_ranks[begin / Self::BLOCK_PER_GROUP]
                + self.block_ranks[begin] as usize);
        let mut begin = 0;
        let mut end = Self::WIDTH;
        while end - begin > 1 {
            let mid = begin + (end - begin) / 2;
            if nth_in_block < Self::NUM_ONES[pattern as usize * Self::WIDTH + mid] {
                end = mid;
            } else {
                begin = mid;
            }
        }

        assert!(pattern & (1 << begin) > 0);

        Some(idx * Self::WIDTH + begin)
    }
}

// 参考: https://miti-7.hatenablog.com/entry/2018/04/28/152259
struct WaveletMatrix<B: BitVector, const H: usize> {
    bits: Vec<B>,
}

type Value = u64;
#[allow(dead_code)]
impl<B: BitVector, const H: usize> WaveletMatrix<B, H> {
    fn new(values: &[Value]) -> WaveletMatrix<B, H> {
        let n = values.len();
        if let Some(u) = (1 as Value).checked_shl(H as u32) {
            assert!(values.iter().all(|&x| x < u));
        }

        let bits = (0..H)
            .rev()
            .scan(values.to_vec(), |values, i| {
                let bv = B::new(values.iter().map(|&x| x & (1 << i) > 0));

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
            if val & (1 << (H - 1 - i)) == 0 {
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

    fn is_over_upper_bound(val: Value) -> bool {
        if let Some(u) = (1 as Value).checked_shl(H as u32) {
            val >= u
        } else {
            false
        }
    }

    // [begin, end)でのval未満の値の出現回数
    // O(W)
    fn count_below(&self, begin: usize, end: usize, val: Value) -> usize {
        assert!(begin <= end);

        if Self::is_over_upper_bound(val) {
            return end - begin;
        }

        let mut num = 0;
        let mut begin = begin;
        let mut end = end;

        for i in 0..H {
            let bv = &self.bits[i];
            let pos = 1 << (H - 1 - i);

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
        assert!(begin <= end);
        assert!(lower <= upper);

        if Self::is_over_upper_bound(upper) {
            return (end - begin) - self.count_below(begin, end, lower);
        }

        let mut num = 0;
        let mut begin_l = begin;
        let mut end_l = end;
        let mut begin_u = begin;
        let mut end_u = end;

        for i in 0..H {
            let bv = &self.bits[i];
            let pos = 1 << (H - 1 - i);

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

            assert!(num + num_u >= num_l);
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
    use rand::{Rng, SeedableRng, rngs::SmallRng};

    #[test]
    fn test_bit_vector_by_ranks() {
        test_bv::<BitVectorByRanks>();
    }

    #[test]
    fn test_bit_set_vector() {
        test_bv::<BitSetVector>();
    }

    fn test_bv<BV: BitVector>() {
        test_bv_rank::<BV>();
        test_bv_rank_random::<BV>();
        test_bv_select::<BV>();
        test_bv_select_random::<BV>();
    }

    fn test_bv_rank<BV: BitVector>() {
        let v = [false, false, true, false, false, true, false];
        let bv = BV::new(v);

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

    fn test_bv_rank_random<BV: BitVector>() {
        let mut rng = SmallRng::seed_from_u64(42);

        for n in 1000..3000 {
            let v = std::iter::repeat_with(|| rng.random::<bool>())
                .take(n)
                .collect::<Vec<_>>();
            let bv = BV::new(v.iter().copied());

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
            assert_eq!(rank0, bv.num0());
            assert_eq!(rank1, bv.num1());
        }
    }

    fn test_bv_select<BV: BitVector>() {
        let v = [false, false, true, false, false, true, false];
        let bv = BV::new(v);

        for i in 0..=bv.num0() {
            assert_eq!(v.iter().positions(|&x| !x).nth(i), bv.select0(i), "nth={i}");
        }

        for i in 0..=bv.num1() {
            assert_eq!(v.iter().positions(|&x| x).nth(i), bv.select1(i), "nth={i}");
        }
    }

    fn test_bv_select_random<BV: BitVector>() {
        let mut rng = SmallRng::seed_from_u64(42);

        for n in 1000..3000 {
            let v = std::iter::repeat_with(|| rng.random::<bool>())
                .take(n)
                .collect::<Vec<_>>();
            let bv = BV::new(v.iter().copied());

            let mut rank0 = 0;
            let mut rank1 = 0;
            for i in 0..v.len() {
                if v[i] {
                    assert_eq!(bv.select1(rank1), Some(i));

                    rank1 += 1;
                } else {
                    assert_eq!(bv.select0(rank0), Some(i));

                    rank0 += 1;
                }
            }

            for i in 0..10 {
                assert_eq!(bv.select0(rank0 + i), None);
                assert_eq!(bv.select1(rank1 + i), None);
            }
        }
    }

    #[test]
    fn test_wv_with_bit_vector_by_ranks() {
        test_wv::<BitVectorByRanks>();
    }

    #[test]
    fn test_wv_with_bit_set_vector() {
        test_wv::<BitSetVector>();
    }

    fn test_wv<BV: BitVector>() {
        test_get::<BV>();
        test_rank::<BV>();
        test_rank_below::<BV>();
        test_rank_between::<BV>();
        test_select::<BV>();
        test_quantile::<BV>();
    }

    fn test_get<BV: BitVector>() {
        let a = [1, 100, 34, 22, 9, 8, 77777, 6, 5, 34, 22, 9, 1, 4];
        let wm = WaveletMatrix::<BV, 64>::new(&a);

        for i in 0..a.len() {
            assert_eq!(a[i], wm.get(i));
        }
    }

    fn test_rank<BV: BitVector>() {
        let a = [1, 100, 34, 22, 9, 8, 77777, 6, 5, 34, 22, 9, 1, 4];
        let wm = WaveletMatrix::<BV, 64>::new(&a);

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

    fn test_rank_below<BV: BitVector>() {
        let a = [1, 100, 34, 22, 9, 8, 77777, 6, 5, 34, 22, 9, 1, 4];
        let wm = WaveletMatrix::<BV, 64>::new(&a);

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

    fn test_rank_between<BV: BitVector>() {
        let a = [1, 100, 34, 22, 9, 8, 77777, 6, 5, 34, 22, 9, 1, 4];
        let wm = WaveletMatrix::<BV, 20>::new(&a);

        let test_values = a
            .iter()
            .copied()
            .flat_map(|x| [x.saturating_sub(1), x, x + 1])
            .sorted()
            .dedup()
            .collect::<Vec<_>>();

        for i in 0..a.len() {
            for j in i + 1..a.len() {
                for (lower, upper) in test_values.iter().copied().tuple_combinations() {
                    let rank = a[i..j].iter().filter(|&&x| lower <= x && x < upper).count();

                    assert_eq!(
                        rank,
                        wm.count_between(i, j, lower, upper),
                        "start={i}, end={j}, lower={lower}, upper={upper}"
                    );
                }
            }
        }
    }

    fn test_select<BV: BitVector>() {
        let a = [1, 100, 34, 22, 9, 8, 77777, 6, 5, 34, 22, 9, 1, 4];
        let wm = WaveletMatrix::<BV, 64>::new(&a);

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

    fn test_quantile<BV: BitVector>() {
        let a = [1, 100, 34, 22, 9, 8, 77777, 6, 5, 34, 22, 9, 1, 4];
        let wm = WaveletMatrix::<BV, 64>::new(&a);

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
