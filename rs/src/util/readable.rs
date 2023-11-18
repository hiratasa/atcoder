use bitset_fixed::BitSet;
use cargo_snippet::snippet;

#[snippet("read_digits")]
#[snippet("read_bitset")]
#[snippet("read_bitset_char")]
use proconio::source::{Readable, Source};

#[snippet("read_digits")]
enum Digits {}

#[snippet("read_digits")]
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

#[snippet("read_bitset")]
enum BitSet01 {}

#[snippet("read_bitset")]
impl Readable for BitSet01 {
    type Output = BitSet;
    fn read<R: std::io::BufRead, S: Source<R>>(source: &mut S) -> BitSet {
        let s = source.next_token_unwrap();

        s.chars()
            .map(|c| match c {
                '0' => false,
                '1' => true,
                _ => unreachable!(),
            })
            .enumerate()
            .fold(BitSet::new(s.len()), |mut bs, (i, x)| {
                bs.set(i, x);
                bs
            })
    }
}

#[snippet("read_bitset_char")]
enum BitSetChar {}

#[snippet("read_bitset_char")]
impl Readable for BitSetChar {
    type Output = BitSet;
    fn read<R: std::io::BufRead, S: Source<R>>(source: &mut S) -> BitSet {
        let s = source.next_token_unwrap();

        s.chars()
            .map(|c| {
                if c == 'F' {
                    false
                } else if c == 'T' {
                    true
                } else {
                    unreachable!()
                }
            })
            .enumerate()
            .fold(BitSet::new(s.len()), |mut bs, (i, x)| {
                bs.set(i, x);
                bs
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proconio::{input, source::auto::AutoSource};

    #[test]
    fn test_digits() {
        let source = AutoSource::from("1234567890");

        input! {
            from source,
            s: Digits,
        };

        assert_eq!(&s, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    }

    #[test]
    fn test_bitset01() {
        let source = AutoSource::from("00011110101");

        input! {
            from source,
            bs: BitSet01,
        };

        let mut expected = BitSet::new(11);
        expected.buffer_mut()[0] = 0b10101111000;
        assert_eq!(&bs, &expected);
    }

    #[test]
    fn test_bitset_char() {
        let source = AutoSource::from("FFFTTTTFTFT");

        input! {
            from source,
            bs: BitSetChar,
        };

        let mut expected = BitSet::new(11);
        expected.buffer_mut()[0] = 0b10101111000;
        assert_eq!(&bs, &expected);
    }
}
