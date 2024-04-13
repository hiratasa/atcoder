use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n - 1],
    };

    println!("{}", -a.iter().sum::<i64>());
}
