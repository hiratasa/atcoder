#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

// vec with some initial value
#[allow(unused_macros)]
macro_rules! vvec {
    ($($x:expr),+; $y:expr; $n:expr) => {{
        let mut v = vec![$y; $n];

        let mut it = v.iter_mut();
        $(
            *it.next().unwrap() = $x;
        )+

        v
    }}
}

#[allow(unused_macros)]
macro_rules! read_cols {
    ($($t:ty),+) => {{
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

        let mut it = line.trim()
            .split_whitespace();

        ($(
            it.next().unwrap().parse::<$t>().ok().unwrap()
        ),+)
    }}
}

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_vec<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

fn main() {
    let (h, w) = read_cols!(usize, usize);

    let a = (0..h).map(|_| read_str()).collect::<Vec<_>>();

    let ((si, sj), (gi, gj), b, ws) = (1..=h).fold(
        (
            (0, 0),
            (0, 0),
            vec![vec![false; w + 2]; h + 2],
            vec![vec![]; 26],
        ),
        |t, i| {
            (1..=w).fold(t, |((si, sj), (gi, gj), mut b, mut ws), j| {
                if a[i - 1][j - 1] != '#' {
                    b[i][j] = true;
                }

                if 'a' <= a[i - 1][j - 1] && a[i - 1][j - 1] <= 'z' {
                    ws[a[i - 1][j - 1] as usize - ('a' as usize)].push((i, j));
                }

                if a[i - 1][j - 1] == 'S' {
                    ((i, j), (gi, gj), b, ws)
                } else if a[i - 1][j - 1] == 'G' {
                    ((si, sj), (i, j), b, ws)
                } else {
                    ((si, sj), (gi, gj), b, ws)
                }
            })
        },
    );

    let mut costs = vec![vec![std::usize::MAX; w + 2]; h + 3];
    costs[h + 2].resize(26, std::usize::MAX);
    let mut q = VecDeque::new();

    costs[si][sj] = 0;
    q.push_back((0, si, sj));

    let deltas = [(-1i64, 0i64), (1i64, 0i64), (0i64, -1i64), (0i64, 1i64)];
    while let Some((cost, i, j)) = q.pop_front() {
        // warp
        if i == h + 2 {
            for &(ni, nj) in &ws[j] {
                if cost < costs[ni][nj] {
                    costs[ni][nj] = cost;
                    q.push_front((cost, ni, nj));
                }
            }
        } else {
            if i == gi && j == gj {
                println!("{}", cost);
                return;
            }

            for delta in &deltas {
                let ni = (i as i64 + delta.0) as usize;
                let nj = (j as i64 + delta.1) as usize;

                // eprintln!("{} {} {}", cost, ni, nj);

                if b[ni][nj] && cost + 1 < costs[ni][nj] {
                    // eprintln!("#{} {} {}", cost, ni, nj);

                    costs[ni][nj] = cost + 1;
                    q.push_back((cost + 1, ni, nj));
                }
            }

            let w = a[i - 1][j - 1];
            if 'a' <= w && w <= 'z' {
                let idx = w as usize - 'a' as usize;

                if cost + 1 < costs[h + 2][idx] {
                    costs[h + 2][idx] = cost + 1;
                    q.push_back((cost + 1, h + 2, idx));
                }
            }
        }
    }

    println!("-1");
}
