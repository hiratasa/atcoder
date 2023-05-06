#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

// vec with some initial value
#[allow(unused_macros)]
macro_rules! vvec {
    ($($x:expr),+; $y:expr; $n:expr) => {{
        let mut v = vec![$y; $n];

        let mut it = v.iter_mut();
        $(
            *it.next().unwrap() = $x;
        )+

        v
    }}
}

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
    let (n, m, k) = read_cols!(usize, usize, usize);

    #[derive(Clone, Copy)]
    enum Log {
        Tweet(usize),
        Follow(usize, usize),
        Unfollow(usize, usize),
    }

    let logs = (0..m)
        .map(|_| {
            let l = read_vec::<String>();

            match l[0].as_str() {
                "t" => Log::Tweet(l[1].parse().unwrap()),
                "f" => Log::Follow(l[1].parse().unwrap(), l[2].parse().unwrap()),
                "u" => Log::Unfollow(l[1].parse().unwrap(), l[2].parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>();

    let mut nums = logs
        .iter()
        .copied()
        .rev()
        .fold(
            (vec![0i64; n], vec![0i64; n]),
            |(mut tl_nums, mut tw_nums), log| {
                match log {
                    Log::Tweet(i) => {
                        tw_nums[i - 1] += 1;
                        tl_nums[i - 1] += 1;
                    }
                    Log::Follow(i, j) => {
                        tl_nums[i - 1] += tw_nums[j - 1];
                        tl_nums[j - 1] += tw_nums[i - 1];
                    }
                    Log::Unfollow(i, j) => {
                        tl_nums[i - 1] -= tw_nums[j - 1];
                        tl_nums[j - 1] -= tw_nums[i - 1];
                    }
                }

                (tl_nums, tw_nums)
            },
        )
        .0;
    nums.sort_by_key(|a| std::cmp::Reverse(*a));

    let ans = nums[k - 1];
    println!("{}", ans);
}
