#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::HashMap;
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

struct BIT<T> {
    len: usize,
    values: Vec<T>,
}

impl<T> BIT<T>
where
    T: std::default::Default
        + std::clone::Clone
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>,
{
    fn new(len: usize) -> BIT<T> {
        BIT {
            len,
            values: vec![T::default(); len],
        }
    }

    // [0, i)の和
    fn sum(&self, i: usize) -> T {
        let mut s = T::default();
        let mut idx = i as i64;

        // values[1] ~ values[i] の和
        // (bは1-indexedなのでこれでOK)
        while idx > 0 {
            s = s + self.values[(idx - 1) as usize].clone();
            idx -= idx & -idx;
        }

        return s;
    }

    // [i, j) の和
    fn sum_between(&self, i: usize, j: usize) -> T {
        self.sum(j) - self.sum(i)
    }

    fn add(&mut self, i: usize, a: T) {
        // 1-indexedに直す
        let mut idx = i as i64 + 1;

        while idx as usize <= self.len {
            self.values[(idx - 1) as usize] = self.values[(idx - 1) as usize].clone() + a.clone();
            idx += idx & -idx;
        }
    }
}

const K: usize = 'z' as usize - 'a' as usize + 1;

fn main() {
    let s: Vec<_> = read_str()
        .into_iter()
        .map(|c| c as usize - 'a' as usize)
        .collect();

    let nums = s.iter().fold([0; K], |mut nums, &c| {
        nums[c] += 1;
        nums
    });

    if nums.iter().filter(|&m| m % 2 > 0).count() > 1 {
        println!("-1");
        return;
    }

    let ans: i64 = s
        .iter()
        .scan((0, vec![vec![]; K]), |(idx, idxs), &c| {
            if nums[c] == 2 * idxs[c].len() + 1 {
                idxs[c].push(s.len() / 2);
            } else if 2 * idxs[c].len() < nums[c] {
                idxs[c].push(*idx);
                *idx += 1;
            } else {
                let opposite_idx = idxs[c][nums[c] - idxs[c].len() - 1];
                idxs[c].push(s.len() - 1 - opposite_idx);
            }
            Some(*idxs[c].last().unwrap())
        })
        .scan(BIT::<i64>::new(s.len()), |bit, idx| {
            bit.add(idx, 1);
            Some(bit.sum_between(idx + 1, s.len()))
        })
        .sum();

    println!("{}", ans);
}
