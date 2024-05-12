use std::collections::VecDeque;

use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        c: [Chars; n],
    };

    let costs = iproduct!([0, n - 1], [0, n - 1])
        .map(|(i0, j0)| {
            let color = c[i0][j0];

            let mut q = VecDeque::new();
            let mut costs = vec![vec![usize::MAX; n]; n];

            q.push_back((i0, j0));
            costs[i0][j0] = 0;
            while let Some((i, j)) = q.pop_front() {
                [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .into_iter()
                    .filter_map(|(di, dj)| {
                        Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?))
                    })
                    .filter(|&(ni, nj)| ni < n && nj < n)
                    .map(|(ni, nj)| {
                        if c[ni][nj] == color {
                            (ni, nj, 0)
                        } else {
                            (ni, nj, 1)
                        }
                    })
                    .filter(|&(ni, nj, cost)| {
                        if costs[i][j] + cost < costs[ni][nj] {
                            costs[ni][nj] = costs[i][j] + cost;
                            true
                        } else {
                            false
                        }
                    })
                    .for_each(|(ni, nj, cost)| {
                        if cost == 0 {
                            q.push_front((ni, nj));
                        } else {
                            q.push_back((ni, nj));
                        }
                    });
            }

            costs
        })
        .collect::<Vec<_>>();

    let ans = iproduct!(0..n, 0..n)
        .map(|(i, j)| costs.iter().map(|cc| cc[i][j]).sum::<usize>() - 1)
        .min()
        .unwrap();

    println!("{ans}");
}
