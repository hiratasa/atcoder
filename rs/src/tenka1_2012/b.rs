#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::iter::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, izip};
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;

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
macro_rules! read_tuple {
    ($($t:ty),+) => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

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
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_col<T: FromStr>(n: usize) -> Vec<T> {
    (0..n).map(|_| read()).collect()
}

#[allow(dead_code)]
fn read_mat<T: FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_row()).collect()
}

#[allow(dead_code)]
fn read_vec<R, F: FnMut() -> R>(n: usize, mut f: F) -> Vec<R> {
    (0..n).map(|_| f()).collect()
}

fn main() {
    let s = read::<String>();

    let cards = s
        .chars()
        .map(|c| c.to_string())
        .coalesce(|prev, current| {
            if current != "S" && current != "H" && current != "D" && current != "C" {
                Ok(prev + &current)
            } else {
                Err((prev, current))
            }
        })
        .map(|card| card.chars().collect_vec())
        // .inspect(|card| eprintln!("{:?}", card))
        .collect_vec();

    let (m, hands) = cards
        .iter()
        .try_fold(
            (BTreeMap::new(), vec![]),
            |(mut picked, mut hands), card| {
                hands.push(card);
                if ['A', '1', 'J', 'Q', 'K'].contains(&card[1]) {
                    *picked.entry(card[0]).or_insert(0) += 1;

                    if *picked.get(&card[0]).unwrap() == 5 {
                        return Err((card[0], hands));
                    }
                }

                Ok((picked, hands))
            },
        )
        .unwrap_err();

    let ans = if hands.len() == 5 {
        "0".to_owned()
    } else {
        hands
            .iter()
            .filter(|card| card[0] != m || (card[1].is_digit(10) && card[1] != '1'))
            .map(|card| card.iter().collect::<String>())
            .join("")
    };
    println!("{}", ans);
}
