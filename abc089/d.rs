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

fn main() {
    let headers = read_cols::<u64>();
    let h = headers[0];
    let w = headers[1];
    let d = headers[2];

    let mut cell_indexes = Vec::<(i64, i64)>::new();
    cell_indexes.resize((h * w + 1) as usize, (-1, -1));
    for i in 0..h {
        let col = read_cols::<u64>();
        for j in 0..w {
            let n = col[j as usize];
            cell_indexes[n as usize] = (i as i64, j as i64);
        }
    }

    let mut jump_powers = Vec::<i64>::new();
    jump_powers.resize((h * w + 1) as usize, 0);
    for r in 1..d+1 {
        let mut sum = 0;

        let start = cell_indexes[r as usize];
        let mut current = start;
        for i in 1.. {
            let n = r + i * d;
            if n > h * w {
                break;
            }

            let next = cell_indexes[n as usize];
            sum += (current.0 - next.0).abs() + (current.1 - next.1).abs();
            jump_powers[n as usize] = sum;
            current = next;
        }
    }

    let num_queries = read::<u64>();
    for _ in 0..num_queries {
        let headers = read_cols::<usize>();
        let l = headers[0];
        let r = headers[1];

        let power = jump_powers[r] - jump_powers[l];

        println!("{}", power);
    }
}
