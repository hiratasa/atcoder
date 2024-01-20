use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    };

    let x = xy.iter().copied().map(|t| t.0).sum::<usize>();
    let y = xy.iter().copied().map(|t| t.1).sum::<usize>();

    let ans = match x.cmp(&y) {
        std::cmp::Ordering::Less => "Aoki",
        std::cmp::Ordering::Equal => "Draw",
        std::cmp::Ordering::Greater => "Takahashi",
    };

    println!("{ans}");
}
