use proconio::input;

fn main() {
    input! {
        a: usize, b: usize,
    };

    if a == b {
        println!("-1");
    } else {
        println!("{}", 6 - a - b);
    }
}
