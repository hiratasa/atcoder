#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::i64;
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
    fn conv(c: char) -> usize {
        match c {
            'U' => 0,
            'R' => 1,
            'D' => 2,
            'L' => 3,
            _ => unreachable!(),
        }
    };

    let n: usize = read();
    let xyu = (0..n)
        .map(|_| read_cols!(i64, i64, char))
        .collect::<Vec<_>>();

    let ps = (0..4)
        .map(|t| {
            [(1, 0), (0, 1), (1, 1), (1, -1)]
                .iter()
                .copied()
                .map(|(cx, cy): (i64, i64)| {
                    let (dx, dy) = (-cy, cx);

                    xyu.iter().filter(|&&(_, _, u)| conv(u) == t).fold(
                        BTreeMap::new(),
                        |mut v, &(x, y, _)| {
                            let key = cx * x + cy * y;
                            let val = dx * x + dy * y;
                            v.entry(key).or_insert(BTreeSet::new()).insert(val);

                            v
                        },
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // (U, R)
    //  x+y == x'+y'; after y'-y = [(-x'+y')-(-x+y)]/2
    // (L, D)
    //  x+y == x'+y'; after y'-y = [(-x'+y')-(-x+y)]/2
    // (U, L)
    //  x-y == x'-y'; after y'-y = [(x'+y')-(x+y)]/2
    // (R, D)
    //  x-y == x'-y'; after y'-y = [(x'+y')-(x+y)]/2
    // (U, D)
    //  x == x'; after (y'-y)/2
    // (L, R)
    //  y == y'; after (-x'+x)/2

    let ans = [
        ('U', 'R', 2),
        ('L', 'D', 2),
        ('U', 'L', 3),
        ('R', 'D', 3),
        ('U', 'D', 0),
        ('L', 'R', 1),
    ]
    .iter()
    .copied()
    .map(|(c1, c2, i)| {
        let t1 = conv(c1);
        let t2 = conv(c2);
        ps[t1][i]
            .iter()
            .filter_map(|(k, v)| ps[t2][i].get(k).map(|v2| (v, v2)))
            .map(|(v, v2)| {
                v.iter()
                    .filter_map(|x| v2.range(x..).next().map(|x2| x2 - x))
                    .min()
                    .unwrap_or(i64::MAX)
            })
            .min()
            .unwrap_or(i64::MAX)
    })
    .min()
    .unwrap_or(i64::MAX);

    if ans == i64::MAX {
        println!("SAFE");
    } else {
        println!("{}", 5 * ans);
    }
}
