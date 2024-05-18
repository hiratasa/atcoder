use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        sc: [(String, usize); n],
    };

    let sum = sc.iter().map(|(_, r)| *r).sum::<usize>();

    let ans = sc.into_iter().sorted().nth(sum % n).unwrap().0;

    println!("{ans}");
}
