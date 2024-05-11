use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    };

    let c = ab.iter().map(|&(a, b)| b - a).max().unwrap();

    println!("{}", ab.iter().map(|&(a, _)| a).sum::<usize>() + c);
}
