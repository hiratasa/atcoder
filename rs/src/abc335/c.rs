use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, q: usize,
    };

    let mut idx = 0;
    let mut coords = (1..=n).map(|i| (i as i64, 0i64)).collect::<Vec<_>>();
    for _ in 0..q {
        input! {
            ty: usize,
        };

        if ty == 1 {
            input! {
                c: char,
            };

            let mut current = coords[idx];
            match c {
                'R' => {
                    current.0 += 1;
                }
                'L' => {
                    current.0 -= 1;
                }
                'U' => {
                    current.1 += 1;
                }
                'D' => {
                    current.1 -= 1;
                }
                _ => unreachable!(),
            }

            idx = (idx + n - 1) % n;
            coords[idx] = current;
        } else {
            input! {
                p: Usize1,
            };

            let (x, y) = coords[(idx + p) % n];
            println!("{x} {y}");
        }
    }
}
