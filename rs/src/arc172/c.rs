use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        c: Chars,
    };

    let c0 = c[0];
    let d = c[1..]
        .iter()
        .map(|&x| if x == c0 { 1i64 } else { -1i64 })
        .collect::<Vec<_>>();

    let mut s = 0;
    let mut ans = 1usize;
    for (i, &x) in d.iter().enumerate() {
        s += x;

        if s == 0 {
            ans += 1;
        }
        if s == -1 && i + 1 < n - 1 && d[i + 1] < 0 {
            ans += 1;
        }
    }
    if s < 0 {
        ans += 1;
    }

    println!("{ans}");
}
