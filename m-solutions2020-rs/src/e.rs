#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

use bitset_fixed::BitSet;

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
fn read_vec<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

fn main() {
    let n: usize = read();

    let xyp: Vec<_> = (0..n).map(|_| read_cols!(i64, i64, i64)).collect();

    let mut xs = xyp
        .iter()
        .enumerate()
        .map(|(i, &(x, _, _))| (x, i))
        .collect::<Vec<_>>();
    xs.sort();

    let mut ys = xyp
        .iter()
        .enumerate()
        .map(|(i, &(_, y, _))| (y, i))
        .collect::<Vec<_>>();
    ys.sort();

    let mut ans = vec![std::i64::MAX; n + 1];

    let dist0 = xyp
        .iter()
        .map(|(x, y, _)| min(x.abs(), y.abs()))
        .collect::<Vec<_>>();

    let mut x_dists = vec![vec![0; n]; 1 << n];
    let mut y_dists = vec![vec![0; n]; 1 << n];
    for u in 0..1 << n {
        let bs = {
            let mut bs = BitSet::new(n);
            bs.buffer_mut()[0] = u;
            bs
        };

        {
            let mut dist = dist0.clone();

            {
                let mut last = -100000;
                for &(x, i) in &xs {
                    if bs[i] {
                        dist[i] = 0;
                        last = x;
                    } else {
                        dist[i] = min(dist[i], x - last);
                    }
                }
            }

            {
                let mut last = 100000;
                for &(x, i) in xs.iter().rev() {
                    if bs[i] {
                        dist[i] = 0;
                        last = x;
                    } else {
                        dist[i] = min(dist[i], last - x);
                    }
                }
            }

            x_dists[u as usize] = dist;
        }

        {
            let mut dist = dist0.clone();

            {
                let mut last = -100000;
                for &(y, i) in &ys {
                    if bs[i] {
                        dist[i] = 0;
                        last = y;
                    } else {
                        dist[i] = min(dist[i], y - last);
                    }
                }
            }

            {
                let mut last = 100000;
                for &(y, i) in ys.iter().rev() {
                    if bs[i] {
                        dist[i] = 0;
                        last = y;
                    } else {
                        dist[i] = min(dist[i], last - y);
                    }
                }
            }

            y_dists[u as usize] = dist;
        }
    }

    for u in 0..1 << n {
        let bs = {
            let mut bs = BitSet::new(n);
            bs.buffer_mut()[0] = u;
            bs
        };

        let k = bs.count_ones();

        let mut t = u;
        loop {
            let xt = u & t;
            let yt = u & !t;

            let dist: i64 = x_dists[xt as usize]
                .iter()
                .zip(y_dists[yt as usize].iter())
                .map(|(x, y)| min(x, y))
                .enumerate()
                .map(|(i, d)| d * xyp[i].2)
                .sum();

            ans[k as usize] = min(ans[k as usize], dist);

            if t == 0 {
                break;
            }

            t = (t - 1) & u;
        }
    }

    for a in ans {
        println!("{}", a);
    }
}
