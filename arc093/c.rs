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

fn main() {
    let n = read::<usize>();
    let pos = read_vec::<i64>();

    assert_eq!(pos.len(), n);

    let travel_pos = {
        let mut travel_pos = vec![0];
        travel_pos.append(&mut pos.clone());
        travel_pos.push(0 as i64);
        travel_pos
    };

    let total_cost = travel_pos.iter().skip(1).scan(0, |current, &x| {
        let cost = (*current - x).abs();
        *current = x;
        Some(cost)
    }).sum::<i64>();

    for i in 0 .. n {
        let prev = travel_pos[i];
        let p = travel_pos[i + 1];
        let next = travel_pos[i + 2];
        let cost = total_cost - (prev - p).abs() - (p - next).abs() + (prev - next).abs();
        println!("{}", cost);
    }

}