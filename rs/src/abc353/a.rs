use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    };

    if let Some(i) = h.iter().skip(1).position(|x| *x > h[0]) {
        println!("{}", i + 2);
    } else {
        println!("-1");
    }
}
