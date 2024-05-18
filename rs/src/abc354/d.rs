use proconio::input;

fn main() {
    input! {
        a: i64, b: i64, c: i64, d: i64,
    };

    const M: i64 = 1000000000;
    let a = a + M;
    let b = b + M;
    let c = c + M;
    let d = d + M;

    let ans = calc(c, d) + calc(a, b) - calc(a, d) - calc(c, b);

    println!("{ans}");
}

fn calc(a: i64, b: i64) -> i64 {
    let aa = a / 4;
    let ra = a % 4;
    let bb = b / 2;
    let rb = b % 2;

    let ans0 = aa * bb * 8 + aa * rb * 4;
    let ans1 = match ra {
        0 => 0,
        1 | 2 => ra * bb * 3,
        3 => bb * 7,
        _ => unreachable!(),
    };
    let ans2 = match (ra, rb) {
        (0, _) | (_, 0) => 0,
        (1, 1) => 2,
        (2, 1) => 3,
        (3, 1) => 3,
        _ => unreachable!(),
    };

    ans0 + ans1 + ans2
}
