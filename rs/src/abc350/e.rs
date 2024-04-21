use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize, a: usize, x: f64, y: f64,
    };

    println!("{}", solve(n, a, x, y, &mut FxHashMap::default()));
}

fn solve(n: usize, a: usize, x: f64, y: f64, memo: &mut FxHashMap<usize, f64>) -> f64 {
    if n == 0 {
        return 0.0;
    }

    if let Some(&m) = memo.get(&n) {
        return m;
    }

    let m0 = solve(n / a, a, x, y, memo) + x;
    let m1 = (2..=6).map(|i| solve(n / i, a, x, y, memo)).sum::<f64>() / 5.0 + 1.2 * y;

    let m = f64::min(m0, m1);
    memo.insert(n, m);

    m
}
