use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let adjs = ab.into_iter().fold(vec![vec![]; n], |mut adjs, (a, b)| {
        adjs[a].push(b);
        adjs[b].push(a);
        adjs
    });

    let ans = (0..n)
        .scan(vec![false; n], |visited, v| {
            let (x, y) = calc(&adjs, visited, v);
            assert_eq!(y % 2, 0);

            Some(x * x.saturating_sub(1) / 2 - y / 2)
        })
        .sum::<usize>();

    println!("{ans}");
}

fn calc(adjs: &[Vec<usize>], visited: &mut [bool], v: usize) -> (usize, usize) {
    if visited[v] {
        return (0, 0);
    }

    visited[v] = true;

    let mut x = 1;
    let mut y = 0;
    for &u in &adjs[v] {
        let (dx, dy) = calc(adjs, visited, u);

        x += dx;
        y += dy + 1;
    }

    (x, y)
}
