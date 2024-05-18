use proconio::input;

fn main() {
    input! {
        h: usize,
    };

    let ans = (0u32..)
        .scan(0usize, |t, i| {
            *t = t.saturating_add(2usize.saturating_pow(i));
            Some(*t)
        })
        .position(|t| t > h)
        .unwrap()
        + 1;

    println!("{ans}");
}
