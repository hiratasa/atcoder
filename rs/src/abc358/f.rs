fn main() {
    input! {
        n: usize, m: usize, k: usize,
    };

    if let Some((h_walls, v_walls)) = solve(n, m, k) {
        assert_eq!(h_walls.len(), n - 1);
        assert_eq!(h_walls[0].len(), m);
        assert_eq!(v_walls.len(), n);
        assert_eq!(v_walls[0].len(), m - 1);

        println!("Yes");

        println!("{}S+", repeat_n('+', 2 * m - 1).join(""));

        for i in 0..n {
            print!("+");
            for j in 0..m - 1 {
                print!("o");
                if v_walls[i][j] {
                    print!("|");
                } else {
                    print!(".");
                }
            }
            println!("o+");

            if i == n - 1 {
                continue;
            }
            print!("+");
            for j in 0..m {
                if h_walls[i][j] {
                    print!("-");
                } else {
                    print!(".");
                }
                print!("+");
            }
            println!();
        }

        println!("{}G+", repeat_n('+', 2 * m - 1).join(""));
    } else {
        println!("No");
    }
}

fn solve(n: usize, m: usize, k: usize) -> Option<(Vec<Vec<bool>>, Vec<Vec<bool>>)> {
    if k < n {
        return None;
    }

    if k % 2 != n % 2 {
        return None;
    }

    if m == 1 {
        assert_eq!(k, n);

        return Some((vec![vec![false]; n - 1], vec![vec![]; n]));
    }

    let mut h_walls = vec![vec![false; m]; n - 1];
    let mut v_walls = vec![vec![false; m - 1]; n];

    for i in 0..n / 2 {
        for j in 1..m {
            h_walls[2 * i][j] = true;
        }
        if 2 * i + 1 < h_walls.len() {
            for j in 0..m - 1 {
                h_walls[2 * i + 1][j] = true;
            }
        }
    }

    let r = k - n;
    if n % 2 == 0 || r <= (n - 1) * (m - 1) {
        let c = r / (2 * (m - 1));
        let d = r % (2 * (m - 1)) / 2;

        let (c, d) = if c > 0 && d == 0 {
            (c - 1, m - 1)
        } else {
            (c, d)
        };

        if 2 * c < n - 1 {
            h_walls[2 * c][m - 1 - d] = false;
        }
        if d + 2 <= m {
            v_walls[2 * c][m - 2 - d] = true;
            if 2 * c + 1 < n {
                v_walls[2 * c + 1][m - 2 - d] = true;
            }
        }

        for i in 2 * (c + 1)..n {
            v_walls[i][m - 2] = true;
        }
        for i in 2 * (c + 1)..n {
            if i < h_walls.len() {
                h_walls[i][m - 1] = false;
            }
        }
    } else {
        let u = r - (n - 1) * (m - 1);
        assert_eq!(u % 2, 0);

        let j0 = (m + 1) % 2;

        for idx in 0..u / 2 {
            let j = 2 * idx + j0;

            h_walls[n - 2][j] = false;
            h_walls[n - 2][j + 1] = false;

            if j > 0 {
                v_walls[n - 1][j - 1] = true;
            }
            v_walls[n - 2][j] = true;
            v_walls[n - 1][j + 1] = true;
        }
        v_walls[n - 1][m - 2] = true;
    }

    Some((h_walls, v_walls))
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::*,
    mem::{replace, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{repeat_n, *};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
