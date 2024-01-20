use itertools::Itertools;
use proconio::input_interactive;

fn main() {
    input_interactive! {
        n: usize,
    };

    let m = (n - 1).ilog2() as usize + 1;
    println!("{m}");

    for i in 0..m {
        let v = (0..n).filter(|&x| x & (1 << i) > 0).collect::<Vec<_>>();

        println!("{} {}", v.len(), v.iter().map(|i| i + 1).join(" "));
    }

    input_interactive! {
        s: Digits,
    };

    let x = (0..m).filter(|&i| s[i] == 1).map(|i| 1 << i).sum::<usize>();

    println!("{}", x + 1);
}

use proconio::source::{Readable, Source};
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
