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
    let n = read::<String>();

    let digits: Vec<i32> = n.chars().map(|v| v.to_digit(10).unwrap() as i32).collect();
    let digits_sum = digits.iter().sum();
    let nine_sum = 9 * ((n.len() as i32) - 1);
    let mut ans = max(digits_sum, nine_sum);
    for i in 0..digits.len() - 1 {
        if digits[i] == 0 {
            continue;
        }

        let sum = digits
            .iter()
            .map(|v| *v)
            .enumerate()
            .map(|(i_digit, v)| {
                if i_digit > i {
                    9
                } else if i_digit == i {
                    v - 1
                } else {
                    v
                }
            })
            .sum();

        if sum > ans {
            ans = sum;
        }
    }

    println!("{}", ans);
}
