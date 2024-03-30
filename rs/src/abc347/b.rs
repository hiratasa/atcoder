use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let n = s.len();

    println!(
        "{}",
        (0..=n)
            .tuple_combinations()
            .map(|(i, j)| s[i..j].iter().collect::<String>())
            .sorted()
            .dedup()
            .count()
    );
}
