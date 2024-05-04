use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };

    let xy = xy
        .into_iter()
        .map(|(x, y)| (x + y, x - y))
        .collect::<Vec<_>>();

    let xy0 = xy
        .iter()
        .copied()
        .filter(|&(x, y)| x % 2 == 0)
        .collect::<Vec<_>>();
    let xy1 = xy
        .iter()
        .copied()
        .filter(|&(x, y)| x % 2 == 1)
        .collect::<Vec<_>>();

    let ans = [xy0, xy1]
        .into_iter()
        .map(|xy| {
            let sx = xy
                .iter()
                .copied()
                .map(|t| t.0)
                .sorted()
                .enumerate()
                .map(|(i, x)| x * (i as i64 - (xy.len() - i - 1) as i64))
                .sum::<i64>();
            let sy = xy
                .iter()
                .copied()
                .map(|t| t.1)
                .sorted()
                .enumerate()
                .map(|(i, x)| x * (i as i64 - (xy.len() - i - 1) as i64))
                .sum::<i64>();

            (sx + sy) / 2
        })
        .sum::<i64>();

    println!("{ans}");
}
