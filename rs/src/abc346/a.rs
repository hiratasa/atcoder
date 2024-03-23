use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    println!("{}", a.iter().tuple_windows().map(|(x, y)| x * y).join(" "));
}
