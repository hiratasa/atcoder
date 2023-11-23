use std::iter::once;

type Block = u8;
type BlockRank = u8;

// NOTE: サイズが十分に小さいケースでは、BlockRankを十分に大きい型にしてblock_group_ranksを廃止したほうが速い
struct BitVector {
    len: usize,
    blocks: Vec<Block>,
    block_ranks: Vec<BlockRank>,
    block_group_ranks: Vec<usize>,
}

impl BitVector {
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

    fn new(iter: impl IntoIterator<Item = bool>) -> BitVector {
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

        BitVector {
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
struct WaveletMatrix<const H: usize> {
    bits: Vec<BitVector>,
}

type Value = u64;
#[allow(dead_code)]
impl<const H: usize> WaveletMatrix<H> {
    fn new(values: &[Value]) -> WaveletMatrix<H> {
        let n = values.len();
        if let Some(u) = (1 as Value).checked_shl(H as u32) {
            assert!(values.iter().all(|&x| x < u));
        }

        let bits = (0..H)
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
    use rand::{rngs::SmallRng, Rng, SeedableRng};

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
    fn test_bv_rank_random() {
        let mut rng = SmallRng::seed_from_u64(42);

        for n in 1000..3000 {
            let v = std::iter::repeat_with(|| rng.gen::<bool>())
                .take(n)
                .collect::<Vec<_>>();
            let bv = BitVector::new(v.iter().copied());

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
    fn test_bv_select_random() {
        let mut rng = SmallRng::seed_from_u64(42);

        for n in 1000..3000 {
            let v = std::iter::repeat_with(|| rng.gen::<bool>())
                .take(n)
                .collect::<Vec<_>>();
            let bv = BitVector::new(v.iter().copied());

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
    fn test_get() {
        let a = [1, 100, 34, 22, 9, 8, 77777, 6, 5, 34, 22, 9, 1, 4];
        let wm = WaveletMatrix::<64>::new(&a);

        for i in 0..a.len() {
            assert_eq!(a[i], wm.get(i));
        }
    }

    #[test]
    fn test_rank() {
        let a = [1, 100, 34, 22, 9, 8, 77777, 6, 5, 34, 22, 9, 1, 4];
        let wm = WaveletMatrix::<64>::new(&a);

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
        let wm = WaveletMatrix::<64>::new(&a);

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
    fn test_rank_between() {
        let a = [1, 100, 34, 22, 9, 8, 77777, 6, 5, 34, 22, 9, 1, 4];
        let wm = WaveletMatrix::<20>::new(&a);

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

    #[test]
    fn test_select() {
        let a = [1, 100, 34, 22, 9, 8, 77777, 6, 5, 34, 22, 9, 1, 4];
        let wm = WaveletMatrix::<64>::new(&a);

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
        let wm = WaveletMatrix::<64>::new(&a);

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

    #[test]
    fn test_performance() {
        let mut rng = SmallRng::seed_from_u64(42);

        let analysis = |name: &str, times: &[u128]| {
            let n = times.len() as u128;
            let s = times.iter().sum::<u128>();
            let s2 = times.iter().map(|x| x * x).sum::<u128>();

            println!(
                "#{name}: {}ms +- {}ms",
                s / n,
                (s2 as f64 / n as f64 - (s as f64 / n as f64).powi(2)).sqrt()
                    / ((n - 1) as f64).sqrt()
            );
        };

        {
            let mut measure = |n: usize, m: usize, constraint: bool| {
                let a = (0..n)
                    .map(|i| {
                        if constraint {
                            rng.gen_range(i * m / n..m)
                        } else {
                            rng.gen_range(0..m)
                        }
                    })
                    .map(|x| x as u64)
                    .collect::<Vec<_>>();
                let wv = WaveletMatrix::<9>::new(&a);
                let queries = (0..10_000_000)
                    .map(|_| {
                        let (b, e, v0, v1) = (
                            rng.gen_range(0..n),
                            rng.gen_range(0..n),
                            rng.gen_range(0..m),
                            rng.gen_range(0..m),
                        );
                        let (b, e) = (std::cmp::min(b, e), std::cmp::max(b, e) + 1);
                        let (v0, v1) = (std::cmp::min(v0, v1), std::cmp::max(v0, v1) + 1);

                        (b, e, v0 as u64, v1 as u64)
                    })
                    .collect::<Vec<_>>();

                let start = std::time::Instant::now();

                let mut s = 0;
                for (b, e, v0, v1) in queries {
                    s += wv.count_between(b, e, v0, v1);
                }

                println!("{}", s);

                start.elapsed().as_millis()
            };

            const K: u128 = 10;
            let times = (0..K).map(|_| measure(512, 512, false)).collect::<Vec<_>>();
            analysis("9;512", &times);
            let times = (0..K).map(|_| measure(500, 500, false)).collect::<Vec<_>>();
            analysis("9;500", &times);
            let times = (0..K).map(|_| measure(500, 500, true)).collect::<Vec<_>>();
            analysis("9;500t", &times);
            let times = (0..K)
                .map(|_| measure(100000, 500, false))
                .collect::<Vec<_>>();
            analysis("9;100000;500", &times);
        }

        {
            let mut measure = |n: usize, m: usize, constraint: bool| {
                let a = (0..n)
                    .map(|i| {
                        if constraint {
                            rng.gen_range(i * m / n..m)
                        } else {
                            rng.gen_range(0..m)
                        }
                    })
                    .map(|x| x as u64)
                    .collect::<Vec<_>>();
                let wv = WaveletMatrix::<32>::new(&a);
                let queries = (0..10_000_000)
                    .map(|_| {
                        let (b, e, v0, v1) = (
                            rng.gen_range(0..n),
                            rng.gen_range(0..n),
                            rng.gen_range(0..m),
                            rng.gen_range(0..m),
                        );
                        let (b, e) = (std::cmp::min(b, e), std::cmp::max(b, e) + 1);
                        let (v0, v1) = (std::cmp::min(v0, v1), std::cmp::max(v0, v1) + 1);

                        (b, e, v0 as u64, v1 as u64)
                    })
                    .collect::<Vec<_>>();

                let start = std::time::Instant::now();

                let mut s = 0;
                for (b, e, v0, v1) in queries {
                    s += wv.count_between(b, e, v0, v1);
                }

                println!("{}", s);

                start.elapsed().as_millis()
            };

            const K: u128 = 10;
            let times = (0..K).map(|_| measure(512, 512, false)).collect::<Vec<_>>();
            analysis("32;512", &times);
            let times = (0..K)
                .map(|_| measure(100000, 100000, false))
                .collect::<Vec<_>>();
            analysis("32;100000", &times);
            let times = (0..K)
                .map(|_| measure(100000, 100000, true))
                .collect::<Vec<_>>();
            analysis("32;100000t", &times);
            let times = (0..K)
                .map(|_| measure(500, 100000, false))
                .collect::<Vec<_>>();
            analysis("32;500;100000", &times);
            let times = (0..K)
                .map(|_| measure(500, 100000, false))
                .collect::<Vec<_>>();
            analysis("32;512;100000", &times);
        }
    }
}
