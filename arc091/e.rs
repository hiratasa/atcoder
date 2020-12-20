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

fn main() {
    let (n, a, b) = read_cols!(u64, u64, u64);

    // 最長増加列と最長現象列は高々1つしか要素を共有しない
    if a + b - 1 > n {
        println!("-1");
        return;
    }

    if a * b < n {
        println!("-1");
        return;
    }

    // x: 長さAのブロックの数
    // u: 中間1ブロックの長さ(2 <= u < A, もしくは0)
    // y: u == 0 ? 0 : 1
    // z: 長さ1のブロックの数
    let z = if a == 1 {
        0
    } else {
        (a * b - n) / (a - 1)
    };
    let (u, y) = if a == 1 {
        (0, 0)
    } else {
        match (a * b - n) % (a - 1) {
            0 => (0, 0),
            r => (a - r, 1),
        }
    };
    let x = b - z - y;

    let get_index = |i, j| {
        if i < x {
            Some(i * a + j)
        } else if i < x + y {
            if j >= u {
                None
            } else {
                Some(x * a + j)
            }
        } else {
            if j > 0 {
                None
            } else {
                Some(x * a + u * y + (i - x - y))
            }
        }
    };

    let mut ans = Vec::<Option<u64>>::new();
    ans.resize(n as usize, None);

    {
        let mut set_ans = |i, j, v| {
            let index = get_index(i, j);
            assert!(index.is_some());
            assert!(index.unwrap() < n);
            assert!(ans[index.unwrap() as usize].is_none());
            ans[index.unwrap() as usize] = Some(v);
        };

        let mut current = 1;
        let mut filled = 0;
        if z > 0 {
            for t in (0..b).rev() {
                set_ans(t, 0, current);
                current += 1;
            }
            filled += 1;
        }
        if y > 0 {
            for s in filled..u {
                for t in (0..x + y).rev() {
                    set_ans(t, s, current);
                    current += 1;
                }
            }
            filled = u;
        }
        for s in filled..a {
            for t in (0..x).rev() {
                set_ans(t, s, current);
                current += 1;
            }
        }
    }

    let mut first = true;
    for n in ans {
        assert!(n.is_some());
        if !first {
            print!(" ");
        }
        print!("{}", n.unwrap());
        first = false;
    }

    println!("");
}
