use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: usize,
    };

    let d = 60 - c.count_ones();
    let e = c.count_ones();

    // f = cが0のbitのうちXとYの両方で1のものの数
    // g = cが1のbitのうちXで1のものの数
    // f + g = a
    // f + (e - g) = b
    // f = (a + b - e) / 2
    // g = (a - b + e) / 2
    if (a + b + e) % 2 > 0 || a + b < e || a + e < b {
        println!("-1");
        return;
    }

    let f = (a + b - e) / 2;
    let g = (a + e - b) / 2;

    if f > d || g > e {
        println!("-1");
        return;
    }

    let f = f as usize;
    let g = g as usize;

    let mut x = 0usize;
    let mut y = 0usize;
    (0..60)
        .filter(|&i| c & (1 << i) == 0)
        .take(f)
        .for_each(|idx| {
            x ^= 1 << idx;
            y ^= 1 << idx;
        });
    (0..60)
        .filter(|&i| c & (1 << i) > 0)
        .take(g)
        .for_each(|idx| {
            x ^= 1 << idx;
        });
    (0..60)
        .filter(|&i| c & (1 << i) > 0)
        .skip(g)
        .for_each(|idx| {
            y ^= 1 << idx;
        });

    assert!(x.count_ones() == a);
    assert!(y.count_ones() == b);
    assert_eq!(x ^ y, c);

    println!("{x} {y}");
}
