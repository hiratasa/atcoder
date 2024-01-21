use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let ans = (1..=9 * 14)
        .map(|r| {
            let mut init = vec![vec![vec![0; r]; r + 1]; 2];
            init[0][0][0] = 1;
            let dp = (0..15).rev().fold(init, |prev, i| {
                let e = 10usize.pow(i);
                let d = n / e % 10;

                let mut next = vec![vec![vec![0; r]; r + 1]; 2];
                for less in 0..2 {
                    for sum in 0..=r {
                        for rr in 0..r {
                            if prev[less][sum][rr] == 0 {
                                continue;
                            }

                            for j in 0..10 {
                                if less == 0 && j > d {
                                    break;
                                }
                                if sum + j > r {
                                    break;
                                }

                                let new_less = less | (j < d) as usize;
                                let new_sum = sum + j;
                                let new_rr = (rr + j * e) % r;

                                next[new_less][new_sum][new_rr] += prev[less][sum][rr];
                            }
                        }
                    }
                }

                next
            });

            dp[0][r][0] + dp[1][r][0]
        })
        .sum::<usize>();

    println!("{ans}");
}
