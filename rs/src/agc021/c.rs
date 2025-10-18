#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
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
    let header = read_cols::<u64>();
    let height = header[0];
    let width = header[1];
    let num_wide = header[2];
    let num_tall = header[3];

    // 配置できる
    // 右隅に縦または横のブロックを置いた配置が可能
    // 右隅に縦ブロックを置いた配置が可能とすると、以下のいずれか
    //    最右列がそれ以外空
    //    最右列でその上に縦ブロックがある
    //

    // height % 2 == 0 && height / 2 <= num_tall
    //  => can be success if (height, width - 1, num_wide, num_tall - height / 2)
}
