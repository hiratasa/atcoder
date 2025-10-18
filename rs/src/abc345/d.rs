use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, h: usize, w: usize,
        ab: [(usize, usize); n],
    };

    let ans = ab.into_iter().permutations(n).any(|rects| {
        (0..1 << n).any(|s| {
            let rects = rects
                .iter()
                .copied()
                .enumerate()
                .map(
                    |(i, (a, b))| {
                        if s & (1 << i) == 0 { (a, b) } else { (b, a) }
                    },
                )
                .collect::<Vec<_>>();

            let mut grid = vec![vec![false; w]; h];
            let mut idx = 0;
            for i in 0..h {
                for j in 0..w {
                    if grid[i][j] {
                        continue;
                    }

                    if idx == n {
                        return false;
                    }

                    let (a, b) = rects[idx];

                    if i + a > h {
                        return false;
                    }

                    if j + b > w {
                        return false;
                    }

                    for ii in 0..a {
                        for jj in 0..b {
                            if grid[i + ii][j + jj] {
                                return false;
                            }

                            grid[i + ii][j + jj] = true;
                        }
                    }
                    idx += 1;
                }
            }

            true
        })
    });

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
