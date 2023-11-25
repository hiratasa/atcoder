use proconio::input;

fn main() {
    input! {
        d: usize,
    };

    let ans = (0usize..)
        .take_while(|&x| x.saturating_sub(1).pow(2) <= d)
        .flat_map(|x| {
            let y0 = (d.saturating_sub(x * x) as f64).sqrt().floor();

            [(x, y0 as usize), (x, y0 as usize + 1)]
        })
        .map(|(x, y)| (x * x + y * y).abs_diff(d))
        .min()
        .unwrap();

    println!("{ans}");
}
