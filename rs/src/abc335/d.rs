use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![vec![None; n]; n];
    solve(&mut ans, 0, 1);

    for row in ans {
        println!(
            "{}",
            row.into_iter()
                .map(|x| x.map_or("T".to_string(), |x| x.to_string()))
                .join(" ")
        );
    }
}

fn solve(grid: &mut [Vec<Option<usize>>], col_offset: usize, start: usize) {
    let n = grid.len();

    if n == 1 {
        return;
    }

    let mut idx = start;

    for i in 0..n - 1 {
        grid[0][col_offset + i] = Some(idx);
        idx += 1;
    }

    for i in 0..n - 1 {
        grid[i][col_offset + n - 1] = Some(idx);
        idx += 1;
    }

    for i in (1..n).rev() {
        grid[n - 1][col_offset + i] = Some(idx);
        idx += 1;
    }

    for i in (1..n).rev() {
        grid[i][col_offset] = Some(idx);
        idx += 1;
    }

    solve(&mut grid[1..n - 1], col_offset + 1, idx);
}
