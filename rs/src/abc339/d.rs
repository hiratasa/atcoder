use std::{collections::VecDeque, mem::replace};

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    solve(&s);
}

fn solve(s: &[Vec<char>]) {
    let n = s.len();

    let ps = s
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().positions(|&c| c == 'P').map(move |j| (i, j)))
        .collect::<Vec<_>>();

    let mut visited = vec![vec![vec![vec![false; n]; n]; n]; n];
    let mut q = VecDeque::new();
    visited[ps[0].0][ps[0].1][ps[1].0][ps[1].1] = true;
    q.push_back((0usize, ps[0], ps[1]));

    while let Some((c, p0, p1)) = q.pop_front() {
        if p0 == p1 {
            println!("{c}");
            return;
        }

        q.extend(
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .map(|(di, dj)| {
                    (
                        Option::zip(
                            p0.0.checked_add_signed(di).filter(|&ni| ni < n),
                            p0.1.checked_add_signed(dj).filter(|&ni| ni < n),
                        )
                        .filter(|&r| s[r.0][r.1] != '#')
                        .unwrap_or(p0),
                        Option::zip(
                            p1.0.checked_add_signed(di).filter(|&ni| ni < n),
                            p1.1.checked_add_signed(dj).filter(|&ni| ni < n),
                        )
                        .filter(|&r| s[r.0][r.1] != '#')
                        .unwrap_or(p1),
                    )
                })
                .filter(|&(q0, q1)| !replace(&mut visited[q0.0][q0.1][q1.0][q1.1], true))
                .map(|(q0, q1)| (c + 1, q0, q1)),
        );
    }

    println!("-1");
}
