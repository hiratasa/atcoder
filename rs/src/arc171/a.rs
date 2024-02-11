use proconio::input;

fn main() {
    input! {
        t: usize,
        cases: [(usize, usize, usize); t],
    };

    cases
        .into_iter()
        .map(|(n, a, b)| {
            if a > n {
                return false;
            }

            let h = (n + 1) / 2;
            let w = n - a;

            b <= (h - a.saturating_sub(n - h)) * w
        })
        .for_each(|ans| {
            if ans {
                println!("Yes");
            } else {
                println!("No");
            }
        })
}
