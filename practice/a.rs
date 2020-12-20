use std::io::*;
use std::str::*;

fn read_line() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line);
    line.trim().to_string()
}

fn read_cols<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line);
    line.split_whitespace().map(|s| s.parse().ok().unwrap()).collect()
}

fn main() {
    let mut acc: i64 = 0;

    for _ in 0..2 {
        for n in read_cols::<i64>() {
            acc += n;
        }
    }

    let s = read_line();

    println!("{} {}", acc, s);
}