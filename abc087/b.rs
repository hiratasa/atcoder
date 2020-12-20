#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::cmp::Ordering;
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

fn count(coins: &HashMap<u32, u32>, total: u32) -> i32 {
    if total == 0 {
        return 1;
    }

    if coins.is_empty() {
        return 0;
    }

    let (next_coin, max) = coins.iter().nth(0).unwrap();

    (0..max + 1)
        .take_while(|n| n * next_coin <= total)
        .fold(0, |total_count, n| {
            let mut next_coins = coins.clone();
            next_coins.remove(next_coin);
            total_count + count(&next_coins, total - n * next_coin)
        })
}

fn main() {
    let n500 = read::<u32>();
    let n100 = read::<u32>();
    let n50 = read::<u32>();
    let price = read::<u32>();

    let mut coins = HashMap::new();
    coins.insert(500, n500);
    coins.insert(100, n100);
    coins.insert(50, n50);

    println!("{}", count(&coins, price));
}
