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

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_cols<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

fn main() {
    let num = read::<u32>();
    let mut points = read_cols::<u32>();

    let mut each_points = Vec::new();
    each_points.resize(2, 0);
    let mut next_player = 0;

    while !points.is_empty() {
        let (i, &p) = points.iter().enumerate().max_by(|kv, kv2| kv.1.cmp(kv2.1)).unwrap();
        each_points[next_player] += p;
        next_player += 1;
        next_player %= 2;
        points.swap_remove(i);
    }

    println!("{}", each_points[0] - each_points[1]);
}
