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

fn dfs(
    n: usize,
    mat: &Vec<Vec<(usize, i32)>>,
    positions: &mut Vec<Option<i32>>,
    visited: &mut Vec<bool>,
) -> bool {
    if visited[n] {
        return true;
    }

    visited[n] = true;
    for &(m, diff) in mat[n].iter() {
        let pos_m = positions[n].unwrap() + diff;
        if let Some(current_pos_m) = positions[m] {
            if current_pos_m != pos_m {
                return false;
            }

            continue;
        }

        positions[m] = Some(pos_m);
        if !dfs(m, mat, positions, visited) {
            return false;
        }
    }

    return true;
}

fn execute() -> bool {
    let header = read_cols::<usize>();
    let n_variables = header[0];
    let n_info = header[1];

    let mut mat = Vec::new();
    mat.resize(n_variables, Vec::<(usize, i32)>::new());

    for _ in 0..n_info {
        let info = read_cols::<i32>();
        let i_left = (info[0] - 1) as usize;
        let i_right = (info[1] - 1) as usize;
        let diff = info[2];
        mat[i_left].push((i_right, diff));
        mat[i_right].push((i_left, -diff));
    }
    let mat = mat;

    let mut visited = Vec::new();
    visited.resize(n_variables, false);

    let mut positions = Vec::<Option<i32>>::new();
    positions.resize(n_variables, None);

    for i in 0..n_variables {
        if visited[i] {
            continue;
        }

        positions[i] = Some(0);

        if !dfs(i, &mat, &mut positions, &mut visited) {
            return false;
        }
    }

    return true;
}

fn main() {
    println!(
        "{}",
        match execute() {
            false => "No",
            true => "Yes",
        }
    );
}
