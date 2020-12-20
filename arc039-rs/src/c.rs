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

#[derive(Clone, Copy)]
enum Dir {
    Left,
    Right,
    Down,
    Up,
}

use Dir::Down;
use Dir::Left;
use Dir::Right;
use Dir::Up;

impl Dir {
    fn from(c: char) -> Dir {
        match c {
            'L' => Left,
            'R' => Right,
            'D' => Down,
            'U' => Up,
            _ => unreachable!(),
        }
    }

    fn idx(self) -> usize {
        match self {
            Left => 0,
            Right => 1,
            Down => 2,
            Up => 3,
        }
    }

    fn delta(self) -> (i64, i64) {
        match self {
            Left => (-1, 0),
            Right => (1, 0),
            Down => (0, -1),
            Up => (0, 1),
        }
    }

    fn opposite(self) -> Dir {
        match self {
            Left => Right,
            Right => Left,
            Down => Up,
            Up => Down,
        }
    }
}

#[derive(Clone)]
struct Neighbors {
    g: Vec<BTreeMap<(i64, i64), (i64, i64)>>,
}

#[allow(dead_code)]
impl Neighbors {
    fn new() -> Neighbors {
        Neighbors {
            g: vec![BTreeMap::new(); 4],
        }
    }

    fn get(&self, (x, y): (i64, i64), dir: Dir) -> (i64, i64) {
        if let Some(&(nx, ny)) = self.g[dir.idx()].get(&(x, y)) {
            (nx, ny)
        } else {
            let delta = dir.delta();
            (x + delta.0, y + delta.1)
        }
    }

    fn set(&mut self, (x, y): (i64, i64), dir: Dir, (nx, ny): (i64, i64)) {
        self.g[dir.idx()].insert((x, y), (nx, ny));
    }
}

fn main() {
    let _k: usize = read();

    let s = read::<String>()
        .chars()
        .map(|c| Dir::from(c))
        .collect::<Vec<_>>();

    let (x, y, _) = s.iter().fold(
        (0i64, 0i64, Neighbors::new()),
        |(x, y, mut neighbors), &d| {
            for &dd in &[Left, Right, Down, Up] {
                let od = dd.opposite();

                let (mx, my) = neighbors.get((x, y), dd);

                neighbors.set((mx, my), od, neighbors.get((x, y), od));
            }

            let (nx, ny) = neighbors.get((x, y), d);

            (nx, ny, neighbors)
        },
    );

    println!("{} {}", x, y);
}
