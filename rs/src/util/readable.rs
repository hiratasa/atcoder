use cargo_snippet::snippet;

#[snippet("read_digits")]
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
}
