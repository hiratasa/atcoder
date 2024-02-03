use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize, w: usize, n: usize,
    };

    let grid = (0..n)
        .fold(
            (vec![vec![false; w]; h], 0, 0, (-1, 0)),
            |(mut grid, i, j, dir), _| {
                grid[i][j] = !grid[i][j];

                let dir = if grid[i][j] {
                    (dir.1, -dir.0)
                } else {
                    (-dir.1, dir.0)
                };

                let (i, j) = (
                    (i + h).wrapping_add_signed(dir.0) % h,
                    (j + w).wrapping_add_signed(dir.1) % w,
                );

                (grid, i, j, dir)
            },
        )
        .0;

    for row in grid {
        println!(
            "{}",
            row.iter()
                .copied()
                .map(|x| if x { '#' } else { '.' })
                .join("")
        );
    }
}
