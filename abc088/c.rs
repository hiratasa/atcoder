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

fn exec() -> bool {
    let a1_col = read_cols::<i32>();
    let a2_col = read_cols::<i32>();
    let a3_col = read_cols::<i32>();

    let mut diff21_iter = a1_col.iter().zip(a2_col).map(|(a1, a2)| a1 - a2);

    let diff21 = diff21_iter.next().unwrap();
    if !diff21_iter.all(|d| d == diff21) {
        return false;
    }

    let mut diff31_iter = a1_col.iter().zip(a3_col).map(|(a1, a3)| a1 - a3);
    let diff31 = diff31_iter.next().unwrap();
    if !diff31_iter.all(|d| d == diff31) {
        return false;
    }

    return true;
}

fn main() {
    if exec() {
        println!("Yes");
    } else {
        println!("No");
    }
}
