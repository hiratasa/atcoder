use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        s: Digits,
    };

    let ans = s
        .into_iter()
        .group_by(|&d| d == 0)
        .into_iter()
        .filter(|(no_plan, _)| !no_plan)
        .map(|(_, it)| {
            let nums = it.fold([0usize; 2], |mut nums, d| {
                nums[d - 1] += 1;
                nums
            });

            nums[0].saturating_sub(m) + nums[1]
        })
        .max()
        .unwrap_or(0);

    println!("{ans}");
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
