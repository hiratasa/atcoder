#[allow(unused_imports)]
use proconio::fastout;
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

#[fastout]
fn main() {
    let n: usize = read();
    let a = read_vec::<usize>();
    let b = read_vec::<usize>();

    let mut idxs_a = a.iter().enumerate().fold(
        BTreeMap::default(),
        |mut nums: BTreeMap<usize, Vec<usize>>, (idx, &aa)| {
            nums.entry(aa).or_insert(vec![]).push(idx);
            nums
        },
    );
    let mut idxs_b = b.iter().enumerate().fold(
        BTreeMap::default(),
        |mut nums: BTreeMap<usize, Vec<usize>>, (idx, &aa)| {
            nums.entry(aa).or_insert(vec![]).push(idx);
            nums
        },
    );

    let mut ss = (0..=n)
        .map(|i| {
            idxs_a.get(&i).map(|is| is.len()).unwrap_or(0)
                + idxs_b.get(&i).map(|is| is.len()).unwrap_or(0)
        })
        .collect::<Vec<_>>();

    let mut by_nums = (1..=n).fold(
        vec![BTreeSet::default(); n + 1],
        |mut by_nums: Vec<BTreeSet<usize>>, i| {
            if !by_nums.is_empty() {
                let s = ss[i];
                if s > n {
                    by_nums.clear();
                } else {
                    by_nums[s].insert(i);
                }
            }

            by_nums
        },
    );

    if by_nums.is_empty() {
        println!("No");
        return;
    }

    let get_diff_elem = |idxs: &BTreeMap<usize, Vec<usize>>, x: usize| {
        *idxs.keys().skip_while(|&&y| y == x).next().unwrap()
    };

    let c = (1..=n)
        .rev()
        .map(|i| {
            let (x, y) = if by_nums[i].len() == 2 {
                let mut it = by_nums[i].iter().copied();
                let x = it.next().unwrap();
                let y = it.next().unwrap();

                if idxs_a.contains_key(&x) {
                    (x, y)
                } else {
                    (y, x)
                }
            } else if by_nums[i].len() == 1 {
                let mut it = by_nums[i].iter().copied();
                let x = it.next().unwrap();

                if idxs_a.contains_key(&x) {
                    (x, get_diff_elem(&idxs_b, x))
                } else {
                    (get_diff_elem(&idxs_a, x), x)
                }
            } else {
                let x = get_diff_elem(&idxs_a, n + 1);

                (x, get_diff_elem(&idxs_b, x))
            };

            let sx = ss[x];
            by_nums[sx].remove(&x);
            by_nums[sx - 1].insert(x);
            ss[x] -= 1;

            let idx = idxs_a.get_mut(&x).unwrap().pop().unwrap();
            if idxs_a.get(&x).unwrap().is_empty() {
                idxs_a.remove(&x);
            }

            let sy = ss[y];
            by_nums[sy].remove(&y);
            by_nums[sy - 1].insert(y);
            ss[y] -= 1;

            idxs_b.get_mut(&y).unwrap().pop();
            if idxs_b.get(&y).unwrap().is_empty() {
                idxs_b.remove(&y);
            }

            (idx, y)
        })
        .fold(vec![0; n], |mut ans, (idx, y)| {
            ans[idx] = y;
            ans
        });

    println!("Yes");

    let mut delim = "";
    for cc in c {
        print!("{}{}", delim, cc);
        delim = " ";
    }
    println!("");
}
