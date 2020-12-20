#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::iter;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

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
    let costs: [usize; 10] = [0, 2, 5, 5, 4, 5, 6, 3, 7, 6];

    let (n, m) = read_cols!(usize, usize);
    let mut a = read_vec::<i64>();

    assert_eq!(a.len(), m);

    a.sort();

    let mut dp = vec![None; 10001];
    dp[0] = Some(0usize);
    let dp = a.iter().fold(dp, |mut dp, &aa| {
        let cost = costs[aa as usize];
        for i in cost..dp.len() {
            let candidate = match dp[i - cost] {
                Some(ref prev) => prev + 1,
                None => {
                    continue;
                }
            };
            dp[i] = match dp[i] {
                Some(current) if candidate > current => Some(candidate),
                None => Some(candidate),
                _ => {
                    continue;
                }
            };
        }
        dp
    });

    let len = dp[n].unwrap();
    let ans = (0..len).fold((String::new(), n), |(s, current), _| {
        let (aa, next) = a.iter()
            .rev()
            .map(|&aa| (aa, costs[aa as usize]))
            .filter(|&(_, c)| c <= current)
            .filter_map(|(aa, c)| dp[current - c].map(|l| (aa, c, l)))
            .find(|&(_, _, l)| l == len - s.len() - 1)
            .map(|(aa, c, _)| (aa, current - c))
            .expect("found");
        (s + &aa.to_string(), next)
    }).0;
    println!("{}", ans);
}
