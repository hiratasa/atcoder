#[allow(unused_imports)]
use bitset_fixed::BitSet;
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
macro_rules! read_tuple {
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
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

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

trait IteratorDpExt: Iterator + Sized {
    fn dp<T, F: FnMut(&Vec<T>, Self::Item) -> T>(self, init: Vec<T>, mut f: F) -> Vec<T> {
        self.fold(init, |mut dp, item| {
            let next = f(&dp, item);
            dp.push(next);
            dp
        })
    }
}

impl<I> IteratorDpExt for I where I: Iterator + Sized {}

fn solve(ax: i64, ay: i64, bx: i64, by: i64, cx: i64, cy: i64) -> i64 {
    let count_x = |x: i64| [ax, bx, cx].iter().filter(|&xx| *xx == x).count() as i64;
    let count_y = |y: i64| [ay, by, cy].iter().filter(|&yy| *yy == y).count() as i64;
    let count_xy = |x: i64, y: i64| {
        [(ax, ay), (bx, by), (cx, cy)]
            .iter()
            .filter(|&&(xx, yy)| xx == x && yy == y)
            .count() as i64
    };

    let mx = min(ax, min(bx, cx));
    let my = min(ay, min(by, cy));

    if mx < 0 {
        let mx = -(mx + 1);
        if my < 0 {
            let my = -(my + 1);
            if mx == 0 && my == 0 {
                count_xy(-1, 0) + count_xy(0, -1) + count_xy(-1, -1)
            } else if mx == my {
                2 + 2 * mx + 1 - count_xy(-mx, -my)
            } else if mx < my {
                2 + 2 * mx + 2 * (my - mx) - 2 + count_y(-my - 1)
            } else {
                2 + 2 * my + 2 * (mx - my) - 2 + count_x(-mx - 1)
            }
        } else {
            if mx == 0 && my == 0 {
                count_x(-1)
            } else if mx == my {
                1 + 2 * mx + count_x(-mx - 1) - 1
            } else if mx < my {
                1 + 2 * mx + 2 * (my - mx) - 2 + count_y(my + 1)
            } else {
                1 + 2 * my + 2 * (mx - my) - 1 + count_x(-mx - 1)
            }
        }
    } else {
        if my < 0 {
            let my = -(my + 1);
            if mx == 0 && my == 0 {
                count_y(-1)
            } else if mx == my {
                1 + 2 * mx + count_y(-my - 1) - 1
            } else if mx < my {
                1 + 2 * mx + 2 * (my - mx) - 1 + count_y(-my - 1)
            } else {
                1 + 2 * my + 2 * (mx - my) - 2 + count_x(mx + 1)
            }
        } else {
            if mx == 0 && my == 0 {
                count_xy(mx + 1, my + 1)
            } else if mx == my {
                2 * mx + 2 - count_xy(mx, my)
            } else if mx < my {
                2 * mx + 1 + 2 * (my - mx) - 2 + count_y(my + 1)
            } else {
                2 * my + 1 + 2 * (mx - my) - 2 + count_x(mx + 1)
            }
        }
    }
}

fn main() {
    let t: usize = read();

    for _ in 0..t {
        let (ax, ay, bx, by, cx, cy) = read_tuple!(i64, i64, i64, i64, i64, i64);

        let ans = solve(ax, ay, bx, by, cx, cy);
        println!("{}", ans);
    }
}

#[test]
fn test() {
    let correct_solve = |ax: i64, ay: i64, bx: i64, by: i64, cx: i64, cy: i64| {
        let mut x = ax + bx + cx;
        let mut y = ay + by + cy;

        x = if x > 0 {
            x - x / 3 - 1
        } else {
            x + (2 - x) / 3 - 1
        };
        y = if y > 0 {
            y - y / 3 - 1
        } else {
            y + (2 - y) / 3 - 1
        };
        if x == 0 && y == 0 {
            return 0;
        }
        if x == 1 && y == 1 {
            return 1;
        };
        max(max(x, -x), max(y, -y)) + if x == y { 1 } else { 0 }
    };

    for ax in -10..=10 {
        for ay in -10..=10 {
            for &dx in &[-1, 1] {
                for &dy in &[-1, 1] {
                    let bx = ax + dx;
                    let by = ay;
                    let cx = ax;
                    let cy = ay + dy;

                    assert_eq!(
                        solve(ax, ay, bx, by, cx, cy),
                        correct_solve(ax, ay, bx, by, cx, cy),
                        "{} {} {} {} {} {}",
                        ax,
                        ay,
                        bx,
                        by,
                        cx,
                        cy
                    );
                }
            }
        }
    }
}
