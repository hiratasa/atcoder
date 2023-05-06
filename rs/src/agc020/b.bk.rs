#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::HashSet;
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

fn find(num_man_in_groups: &[u64], last_num_man: u64) -> Option<(u64, u64)> {
    let mut min_num_man = last_num_man;
    let mut max_num_man = last_num_man;
    for num_man_in_group in num_man_in_groups.iter().rev() {
        let actual_min_man = ((min_num_man - 1) / num_man_in_group + 1) * num_man_in_group;
        if actual_min_man > max_num_man {
            return None;
        }

        min_num_man = actual_min_man;
        max_num_man = (max_num_man / num_man_in_group + 1) * num_man_in_group - 1;
    }

    return Some((min_num_man, max_num_man));
}

fn main() {
    // 00:04:00
    let num_rounds = read::<u64>();
    let num_man_in_groups = read_cols::<u64>();
    let last_num_man = 2;

    assert_eq!(num_rounds as usize, num_man_in_groups.len());

    match find(num_man_in_groups.as_slice(), last_num_man) {
        Some((min, max)) => println!("{} {}", min, max),
        None => println!("-1"),
    };

    // 0:29:22
    // 25min 22sec
    // 0:45:50 fixed
    // +16min 28sec
}
