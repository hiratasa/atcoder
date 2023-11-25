use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let rows = s
        .iter()
        .map(|row| row.iter().copied().filter(|&x| x == 'o').count())
        .collect::<Vec<_>>();
    let cols = (0..n)
        .map(|j| (0..n).map(|i| s[i][j]).filter(|&x| x == 'o').count())
        .collect::<Vec<_>>();

    let ans = iproduct!(0..n, 0..n)
        .filter(|&(i, j)| s[i][j] == 'o')
        .map(|(i, j)| (rows[i] - 1) * (cols[j] - 1))
        .sum::<usize>();

    println!("{ans}");
}
