use std::cmp::min;
use std::iter::once;

use itertools_num::ItertoolsNum;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize, a: i64, b: i64,
        s: Chars,
    };

    let a = min(a, 2 * b);

    let mut s = s
        .into_iter()
        .map(|c| if c == '(' { 1i64 } else { -1i64 })
        .collect::<Vec<_>>();

    let mut sum = s.iter().sum::<i64>();

    let mut ans = 0;
    if sum < 0 {
        for i in 0..2 * n {
            if s[i] < 0 {
                s[i] = 1;
                sum += 2;
                ans += b;
            }

            if sum == 0 {
                break;
            }
        }
    } else if sum > 0 {
        for i in (0..2 * n).rev() {
            if s[i] > 0 {
                s[i] = -1;
                sum -= 2;
                ans += b;
            }

            if sum == 0 {
                break;
            }
        }
    }

    let min = once(0)
        .chain(s.iter().copied())
        .cumsum::<i64>()
        .min()
        .unwrap();

    if min < 0 {
        ans += (-min + 1) / 2 * a;
    }

    println!("{ans}");
}
