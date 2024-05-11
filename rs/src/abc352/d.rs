use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, k: usize,
        p: [Usize1; n],
    };

    let q = p
        .iter()
        .copied()
        .enumerate()
        .fold(vec![0; n], |mut q, (i, j)| {
            q[j] = i;
            q
        });

    let ans = (0..n)
        .scan((VecDeque::new(), VecDeque::new()), |(mins, maxs), i| {
            let pos = q[i];

            while matches!(mins.back(), Some(&x) if x > pos) {
                mins.pop_back();
            }
            mins.push_back(pos);

            while matches!(maxs.back(), Some(&x) if x < pos) {
                maxs.pop_back();
            }
            maxs.push_back(pos);

            if i >= k {
                if p[mins[0]] <= i - k {
                    mins.pop_front();
                }
                if p[maxs[0]] <= i - k {
                    maxs.pop_front();
                }
            }

            if i >= k - 1 {
                Some(Some(maxs[0] - mins[0]))
            } else {
                Some(None)
            }
        })
        .flatten()
        .min()
        .unwrap();

    println!("{ans}");
}
