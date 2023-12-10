use proconio::input;

fn main() {
    input! {
        n: usize, s: usize, k: usize,
        pq: [(usize, usize); n],
    };

    let x = pq.iter().map(|(p, q)| p * q).sum::<usize>();

    let ans = if x >= s { x } else { x + k };

    println!("{ans}");
}
