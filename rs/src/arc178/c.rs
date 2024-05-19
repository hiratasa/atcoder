use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        n: usize, l: usize,
        a: [usize; n],
    };

    const M: usize = 200000;
    let mut init = vec![usize::MAX; M + 1];
    init[0] = 0;
    let dp = (1..=l)
        .take_while(|&i| i <= l - i)
        .map(|i| i * (l - i))
        .take_while(|&x| x <= M)
        .fold(init, |mut dp, x| {
            for j in 0..=M - x {
                dp[j + x] = min(dp[j + x], dp[j].saturating_add(1));
            }

            dp
        });

    for x in a {
        if dp[x] == usize::MAX {
            println!("-1");
        } else {
            println!("{}", dp[x]);
        }
    }
}
