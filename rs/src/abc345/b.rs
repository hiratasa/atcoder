use proconio::input;

fn main() {
    input! {
        x: i64,
    };

    println!("{}", (x + 9).div_euclid(10));
}
