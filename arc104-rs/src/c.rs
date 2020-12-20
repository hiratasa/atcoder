#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
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

fn main() {
    let n: usize = read();

    let ab = (0..n)
        .map(|_| read_cols!(i64, i64))
        .map(|(a, b)| {
            (
                if a > 0 { Some(a as usize - 1) } else { None },
                if b > 0 { Some(b as usize - 1) } else { None },
            )
        })
        .collect::<Vec<_>>();

    #[derive(Clone, Copy, PartialEq)]
    enum Rec {
        On(usize),
        Off(usize),
        Unknown,
    };

    let recs = ab
        .iter()
        .enumerate()
        .fold(vec![Rec::Unknown; 2 * n], |mut recs, (i, &(a, b))| {
            if recs.is_empty() {
                return recs;
            }

            if let Some(m) = a {
                if recs[m] != Rec::Unknown {
                    recs.clear();
                    return recs;
                }
                recs[m] = Rec::On(i);
            }
            if let Some(m) = b {
                if recs[m] != Rec::Unknown {
                    recs.clear();
                    return recs;
                }
                recs[m] = Rec::Off(i);
            }
            recs
        });
    if recs.is_empty() {
        println!("No");
        return;
    }

    let ok = (1..=n).fold(vec![true; n + 1], |mut oks, i| {
        oks[i] = (1..=i).filter(|j| oks[i - j]).any(|j| {
            ((2 * i - 2 * j)..(2 * i - j)).all(|l| {
                let u = l + j;

                match &recs[l] {
                    &Rec::On(k) => match &recs[u] {
                        &Rec::On(_) => false,
                        &Rec::Off(k2) if k == k2 => true,
                        &Rec::Off(_) => false,
                        &Rec::Unknown => ab[k].1.is_none(),
                    },
                    &Rec::Off(_) => false,
                    &Rec::Unknown => match &recs[u] {
                        &Rec::On(_) => false,
                        &Rec::Off(k) => ab[k].0.is_none(),
                        &Rec::Unknown => true,
                    },
                }
            })
        });
        oks
    })[n];

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
