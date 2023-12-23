use proconio::input;

fn main() {
    input! {
        a: i64, m: i64, l: i64, r: i64
    };

    println!("{}", (r - a).div_euclid(m) - (l - 1 - a).div_euclid(m));
}
