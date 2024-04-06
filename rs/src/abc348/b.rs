use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };

    for i in 0..n {
        let (x, y) = xy[i];
        println!(
            "{}",
            (0..n)
                .rev()
                .max_by_key(|&j| {
                    let (x1, y1) = xy[j];

                    (x - x1).pow(2) + (y - y1).pow(2)
                })
                .unwrap()
                + 1
        );
    }
}
