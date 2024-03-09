use proconio::input;

fn main() {
    input! {
        a: usize, b: usize,
    };

    println!("{}", (0..=9).find(|&x| x != a + b).unwrap());
}
