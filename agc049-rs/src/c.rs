#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
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
    let n: usize = read();

    let a = read_vec::<i64>();
    let b = read_vec::<i64>();

    let oks = (0..n)
        .map(|i| (a[i], b[i]))
        .filter(|(a, b)| a > b)
        .collect::<Vec<_>>();
    let ngs = (0..n)
        .map(|i| (a[i], b[i]))
        .filter(|(a, b)| a <= b)
        .collect::<Vec<_>>();

    if ngs.is_empty() {
        println!("0");
        return;
    }

    let mut v = oks
        .iter()
        .copied()
        .fold(vec![(0, 0)], |mut v: Vec<(usize, usize)>, (a, b)| {
            let c = a - b;

            let l = ngs.binary_search(&(c, 0)).unwrap_err();
            let r = ngs.binary_search(&(a, 0)).unwrap_err();

            if l == r {
                return v;
            }

            while let Some(&(ll, _rr)) = v.last() {
                if ll <= l {
                    break;
                }
                v.pop();
            }

            if let Some((_ll, rr)) = v.last_mut() {
                if l <= *rr {
                    *rr = r;
                } else {
                    v.push((l, r));
                }
            } else {
                v.push((l, r));
            }

            v
        });

    if let Some(&(ll, rr)) = v.first() {
        if ll == 0 && rr == ngs.len() {
            println!("0");
            return;
        }
    }

    v.push((ngs.len(), ngs.len()));

    let mut u = vec![0; v.len()];
    let mut prev = 0;
    let mut prev_idx = ngs.len();
    for i in (0..v.len()).rev() {
        u[i] = prev + (prev_idx - v[i].1) as i64;
        prev = u[i];
        prev_idx = v[i].0;
    }

    let mut ans = prev;

    for (i, &(a, b)) in ngs.iter().enumerate() {
        let t = b - a + 1;

        let idx = v
            .binary_search_by_key(&(i + 2), |&(_, r)| r)
            .unwrap_or_else(|idx| idx);
        if i == ngs.len() - 1 {
            ans = min(ans, t);
        } else if i + 1 >= v[idx].0 {
            ans = min(ans, max(t, u[idx]));
        } else {
            ans = min(ans, max(t, u[idx] + (v[idx].0 - (i + 1)) as i64));
        }
    }

    println!("{}", ans);
}
