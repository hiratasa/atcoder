#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;
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

fn execute(red: &Vec<(usize, usize)>, blue: &Vec<(usize, usize)>, used_blue: &mut Vec<bool>) -> usize {
    let mut ans = 0;
    for r in red {
        let mut candidate = blue.iter()
            .map(|&p| p)
            .enumerate()
            .skip_while(|&(_, (x, _))| x < r.0)
            .filter(|&(i, _)| !used_blue[i])
            .filter(|&(_, (_, y))| y > r.1)
            .min_by(|&(_, (_, y1)), &(_, (_, y2))| y1.cmp(&y2));
        match candidate {
            Some((i, _)) => {
                used_blue[i] = true;
                ans += 1;
            }
            None => {}
        }
    }

    ans
}

fn main() {
    let n = read::<usize>();
    let mut red = Vec::new();
    for _ in 0..n {
        let (a, b) = read_cols!(usize, usize);
        red.push((a, b));
    }
    let mut blue = Vec::new();
    for _ in 0..n {
        let (a, b) = read_cols!(usize, usize);
        blue.push((a, b));
    }

    // 右から左
    red.sort_by(|&(x1, _), &(x2, _)| x2.cmp(&x1));
    // 左から右
    blue.sort_by(|&(x1, _), &(x2, _)| x1.cmp(&x2));

    let mut used_blue = Vec::new();
    used_blue.resize(n, false);

    println!("{}", execute(&red, &blue, &mut used_blue));
}
