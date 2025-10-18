#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_cols<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

type BitSetElement = u64;
struct BitSet {
    size: usize,
    storage: Vec<BitSetElement>,
}

impl BitSet {
    fn unit_size() -> usize {
        size_of::<BitSetElement>()
    }

    fn unit_size_bit() -> usize {
        8 * Self::unit_size()
    }

    fn new(size: usize, default: bool) -> BitSet {
        let unit_size_bit = Self::unit_size_bit();
        let storage_size = (size - 1) / unit_size_bit + 1;
        let mut storage = Vec::new();
        storage.resize(storage_size, 0);

        if default {
            for i in 0..storage_size - 1 {
                storage[i] = (-1i8) as BitSetElement;
            }
            storage[storage_size - 1] =
                ((-1i8) as BitSetElement) >> (unit_size_bit - (size - 1) % unit_size_bit - 1);
        }
        BitSet {
            size: size,
            storage: storage,
        }
    }

    fn set(&mut self, pos: usize) {
        self.storage[pos / Self::unit_size_bit()] |= 1 << (pos % Self::unit_size_bit());
    }

    fn unset(&mut self, pos: usize) {
        self.storage[pos / Self::unit_size_bit()] &= !(1 << (pos % Self::unit_size_bit()));
    }

    fn get(&self, pos: usize) -> bool {
        if pos >= self.size {
            return false;
        }
        self.storage[pos / Self::unit_size_bit()] & (1 << (pos % Self::unit_size_bit())) > 0
    }

    /// self.get(i) | other.get(i-shift)
    fn or_with_shift(&self, other: &BitSet, shift: usize) -> BitSet {
        let unit_size_bit = Self::unit_size_bit();
        let shift_elem = shift / unit_size_bit;
        let bit_size = max(self.size, other.size + shift);
        let byte_size = (bit_size - 1) / unit_size_bit + 1;
        let mut storage = Vec::with_capacity(byte_size);

        for i in 0..byte_size {
            storage.push(0);
            if shift_elem + 1 <= i
                && i < other.storage.len() + shift_elem + 1
                && shift % unit_size_bit > 0
            {
                storage[i] |=
                    other.storage[i - 1 - shift_elem] >> (unit_size_bit - shift % unit_size_bit);
            }
            if shift_elem <= i && i < other.storage.len() + shift_elem {
                storage[i] |= other.storage[i - shift_elem] << (shift % unit_size_bit);
            }
            if i < self.storage.len() {
                storage[i] |= self.storage[i];
            }
        }

        BitSet {
            size: bit_size,
            storage: storage,
        }
    }

    fn dump(&self) {
        for (i, x) in self.storage.iter().enumerate() {
            println!("{}: {:b}", i, x);
        }
    }
}

/// Returns the minimum partial sum of nums,
/// greater than or equal to bound.
fn calc_iterative(nums: &[usize], bound: usize, sums: &[usize]) -> usize {
    if bound <= 0 {
        return 0;
    }

    let mut dp = BitSet::new(nums[0] + 1, false);
    dp.set(0);
    dp.set(nums[0]);

    for (_i, &num) in nums.iter().enumerate().skip(1) {
        let next_dp = dp.or_with_shift(&dp, num);
        dp = next_dp;
    }

    for i in bound.. {
        if dp.get(i) {
            return i;
        }
    }

    return 0;
}

fn main() {
    let n = read::<usize>();
    let nums = {
        let mut nums = read_cols::<usize>();
        nums.sort();
        nums
    };
    assert_eq!(n, nums.len());

    // Idea:
    //  When S=a_1+...+a_n and S_i (i=1, ...) is i-th partial sum
    //  in ascending order and S_0 = 0,
    //  S_{2^n-1-i} = S - S_i,
    //  and if S_i <= S/2, then S_{2^n-1-i} >= S/2.
    //  By this fact, minimum S_k where satisfies S_k >= S/2
    //  is the median of partial sums.
    let sums = nums
        .iter()
        .scan(0, |sum, &num| {
            *sum += num;
            Some(*sum)
        })
        .collect::<Vec<_>>();

    let total_sum = sums.last().unwrap();
    // let answer = calc_partial_sum_lower_bound(&nums, (total_sum + 1) / 2, &sums, &mut cache);
    let answer = calc_iterative(&nums, (total_sum + 1) / 2, &sums);

    println!("{}", answer);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn bit_set() {
        let mut b = BitSet::new(256, false);
        b.set(65);
        b.set(190);
        let c = b.or_with_shift(&b, 64);

        b.dump();
        c.dump();

        assert!(!c.get(0));
        assert!(c.get(65));
        assert!(c.get(129));
        assert!(c.get(190));
        assert!(c.get(254));
        assert!(!c.get(270));
    }
}
