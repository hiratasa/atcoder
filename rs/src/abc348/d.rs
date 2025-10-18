use std::{collections::VecDeque, iter::once};

use itertools::{Itertools, iproduct};
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize, w: usize,
        a: [Chars; h],
        n: usize,
        rce: [(Usize1, Usize1, usize); n],
    };

    let start = iproduct!(0..h, 0..w)
        .find(|&(i, j)| a[i][j] == 'S')
        .unwrap();
    let goal = iproduct!(0..h, 0..w)
        .find(|&(i, j)| a[i][j] == 'T')
        .unwrap();

    let g = rce
        .iter()
        .map(|&(r, c, e)| {
            let mut visited = vec![vec![false; w]; h];
            let mut q = VecDeque::new();
            q.push_back((r, c, e));

            while let Some((i, j, ee)) = q.pop_front() {
                if visited[i][j] {
                    continue;
                }

                visited[i][j] = true;

                if ee == 0 {
                    continue;
                }

                q.extend(
                    [(-1, 0), (1, 0), (0, -1), (0, 1)]
                        .into_iter()
                        .filter_map(|(di, dj)| {
                            Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?))
                        })
                        .filter(|&(ni, nj)| ni < h && nj < w)
                        .filter(|&(ni, nj)| a[ni][nj] != '#')
                        .map(|(ni, nj)| (ni, nj, ee - 1)),
                );
            }

            rce.iter()
                .map(|&(r, c, _)| (r, c))
                .chain(once(goal))
                .map(|(i, j)| visited[i][j])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut visited = vec![false; n];
    let mut stack = vec![];
    stack.extend(rce.iter().positions(|&(r, c, _)| (r, c) == start));
    while let Some(i) = stack.pop() {
        if visited[i] {
            continue;
        }
        visited[i] = true;

        if g[i][n] {
            println!("Yes");
            return;
        }

        stack.extend((0..n).filter(|&j| g[i][j]));
    }

    println!("No");
}
