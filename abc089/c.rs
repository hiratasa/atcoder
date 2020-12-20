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
    let n = read::<u64>();
    let chars = vec!['M', 'A', 'R', 'C', 'H'];
    let mut name_counts = HashMap::new();

    for c in &chars {
        name_counts.insert(*c, 0);
    }

    for _ in 0..n {
        let name = read::<String>();
        let c = name.chars().next().unwrap();

        if chars.iter().all(|&cc| cc != c) {
            continue;
        }

        let n = *name_counts.get(&c).unwrap();

        name_counts.insert(c, n + 1);
    }

    let mut ans: u64 = 0;
    for i in 0..chars.len() {
        let n_i = *name_counts.get(&chars[i]).unwrap();
        if n_i == 0 {
            continue;
        }

        for j in i+1..chars.len() {
            let n_j = *name_counts.get(&chars[j]).unwrap();
            if n_j == 0 {
                continue;
            }

            for k in j+1..chars.len() {
                let n_k = *name_counts.get(&chars[k]).unwrap();
                if n_k == 0 {
                    continue;
                }

                ans += n_i * n_j * n_k;
            }
        }
    }

    println!("{}", ans);
}
