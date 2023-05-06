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

use std::f64;

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

fn find_range(i: usize, points: &Vec<(f64, f64)>) -> Option<f64> {
    let num_points = points.len();

    let mut min: f64 = 0.0;
    let mut max = 2.0 * f64::consts::PI;

    for j in 0..num_points {
        if j == i {
            continue;
        }

        let p = points[i];
        let q = points[j];

        let mid = ((p.0 + q.0) / 2.0, (p.1 + q.1) / 2.0);
        let gradient = if p.1 == q.1 {
            f64::INFINITY
        } else {
            -(p.0 - q.0) / (p.1 - q.1)
        };

        let is_upper = p.1 - mid.1 >= gradient * (p.0 - mid.0);
        let r = gradient.atan();

        let (mut r_min, mut r_max) = if is_upper {
            (r, r + f64::consts::PI)
        } else {
            (r - f64::consts::PI, r)
        };

        if r_min < 0.0 {
            r_min += 2.0 * f64::consts::PI;
            r_max += 2.0 * f64::consts::PI;
        }

        if j == 0 || (j == 1 && i == 0) {
            // first
            min = r_min;
            max = r_max;
            continue;
        }

        // find shared
        if r_min <= min {
            if r_max >= max {
                // no update
                continue;
            } else if min <= r_max {
                max = r_max;
                continue;
            } else if max < r_min + 2.0 * f64::consts::PI {
                // no shared
                return None;
            } else {
                min = r_min;
                max -= 2.0 * f64::consts::PI;
                continue;
            }
        } else if r_min <= max {
            min = r_min;
            continue;
        } else if min > r_max - 2.0 * f64::consts::PI {
            // no shared
            return None;
        } else if max >= r_max - 2.0 * f64::consts::PI {
            max = r_max - 2.0 * f64::consts::PI;
            continue;
        } else {
            // no update
            continue;
        }
    }

    Some((max - min) / (2.0 * f64::consts::PI))
}

fn main() {
    let num_points = read::<usize>();
    let mut points = Vec::with_capacity(num_points);
    for _ in 0..num_points {
        let p = read_cols::<f64>();
        points.push((p[0], p[1]));
    }

    for i in 0..num_points {
        match find_range(i, &points) {
            Some(p) => println!("{}", p),
            None => println!("0"),
        }
    }
}
