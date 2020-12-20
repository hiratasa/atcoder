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

fn visit(
    maze: &Vec<Vec<bool>>,
    state: &mut Vec<Vec<usize>>,
    queue: &mut VecDeque<(usize, usize)>,
    i: i32,
    j: i32,
    distance: usize,
) {
    if i < 0 || i >= maze.len() as i32 {
        return;
    }

    if j < 0 || j >= maze[0].len() as i32 {
        return;
    }

    let i = i as usize;
    let j = j as usize;

    if !maze[i][j] {
        return;
    }

    if state[i][j] > distance {
        state[i][j] = distance;
        queue.push_back((i, j));
    }
}

fn bfs(maze: &Vec<Vec<bool>>) -> i32 {
    let mut queue = VecDeque::new();

    let h = maze.len();
    let w = maze[0].len();

    let mut state = Vec::new();
    {
        let mut inner = Vec::new();
        inner.resize(w, usize::max_value());
        state.resize(h, inner);
    }

    state[0][0] = 0;
    queue.push_back((0, 0));
    while let Some((i, j)) = queue.pop_front() {
        let current = state[i][j];
        if i == h - 1 && j == w - 1 {
            // goal
            return current as i32;
        }
        let next = current + 1;
        let i = i as i32;
        let j = j as i32;
        visit(maze, &mut state, &mut queue, i - 1, j, next);
        visit(maze, &mut state, &mut queue, i, j + 1, next);
        visit(maze, &mut state, &mut queue, i + 1, j, next);
        visit(maze, &mut state, &mut queue, i, j - 1, next);
    }

    -1
}

fn count_white(maze: Vec<Vec<bool>>) -> usize {
    let mut count = 0;
    for col in maze {
        for b in col {
            if b {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let headers = read_cols::<u32>();
    let h = headers[0];
    let _w = headers[1];

    let mut maze = Vec::new();
    for _ in 0..h {
        let col = read::<String>();
        let mut col_maze = Vec::new();
        for c in col.chars() {
            if c == '.' {
                col_maze.push(true);
            } else {
                col_maze.push(false);
            }
        }
        maze.push(col_maze);
    }

    let distance = bfs(&maze);

    if distance < 0 {
        println!("-1");
        return;
    }

    println!("{}", count_white(maze) - (distance as usize + 1));
}
