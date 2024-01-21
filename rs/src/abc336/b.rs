use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    println!("{}", n.trailing_zeros());
}
