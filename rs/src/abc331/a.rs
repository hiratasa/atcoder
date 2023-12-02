use proconio::input;

fn main() {
    input! {
        M: usize, D: usize,
        y: usize, m: usize, d: usize,
    };

    let ans = if (m, d) == (M, D) {
        (y + 1, 1, 1)
    } else if d == D {
        (y, m + 1, 1)
    } else {
        (y, m, d + 1)
    };

    println!("{} {} {}", ans.0, ans.1, ans.2);
}
