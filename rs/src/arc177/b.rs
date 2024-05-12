use std::iter::repeat;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: Digits,
    };

    let mut m = n;
    let mut c = 0;
    let mut ans = vec![];
    // let mut t = vec![0; n];
    while m > 0 {
        if s[m - 1] == c {
            m -= 1;
            continue;
        }

        if c == 0 {
            ans.extend(repeat('A').take(m));
            c = 1;
            // for _ in 0..m {
            //     let pos = t.iter().position(|&x| x == 0).unwrap();
            //     t[pos] = 1;
            // }
        } else {
            ans.extend(repeat('B').take(m));
            c = 0;
            // for _ in 0..m {
            //     let pos = t.iter().position(|&x| x == 1).unwrap();
            //     t[pos] = 0;
            // }
        }

        m -= 1;
    }

    // assert_eq!(t, s);

    println!("{}", ans.len());
    println!("{}", ans.iter().join(""));
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
