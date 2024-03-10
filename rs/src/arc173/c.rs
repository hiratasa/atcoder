use std::cmp::min;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };

    let ans = (0..n)
        .map(|i| {
            if i == 0 || i == n - 1 {
                let q = if i == 0 {
                    p.clone()
                } else {
                    let mut q = p.clone();
                    q.reverse();
                    q
                };

                let x = q[0];
                if let Some(j) = (2..n).step_by(2).find(|&j| {
                    let y = q[j - 1];
                    let z = q[j];
                    (y > x) == (z > x)
                }) {
                    return Some(j + 1);
                } else {
                    return None;
                }
            }

            let big = p[i] < p[i + 1];

            let mut left = 0usize;
            let mut right = 0usize;
            for j in 1..n {
                if i < j && i + j >= n {
                    break;
                }

                let mut c = 0;
                if i >= j {
                    let g = if j % 2 == 0 { big } else { !big };

                    if g == (p[i - j] > p[i]) {
                        c += 1;
                        left += 1;
                    }
                } else {
                    c += 1;
                }

                if i + j < n {
                    let g = if j % 2 == 0 { !big } else { big };

                    if g == (p[i + j] > p[i]) {
                        c += 1;
                        right += 1;
                    }
                } else {
                    c += 1;
                }

                if c != 2 {
                    break;
                }
            }

            let ans = if left + right == n - 1 {
                None
            } else if left + 1 <= i && right + 1 <= n - i - 1 {
                Some(min(left / 2, right / 2) * 2 + 3)
            } else if left + 1 > i {
                Some(right / 2 * 2 + 3)
            } else {
                assert!(right + 1 > n - i - 1);
                Some(left / 2 * 2 + 3)
            };

            ans.filter(|&m| m <= n)
        })
        .collect::<Vec<_>>();

    println!(
        "{}",
        ans.iter().map(|x| x.map_or(-1, |x| x as i64)).join(" ")
    );
}
