use proconio::input;

fn main() {
    input! {
        mut l: usize, r: usize,
    };

    let mut ans = vec![];
    while l > 0 && l >> l.trailing_zeros() != r >> l.trailing_zeros() {
        assert!(l < r);

        let i = l.trailing_zeros();

        ans.push((l, l + (1 << i)));

        l += 1 << i;
    }

    while l < r {
        let i = (r & !l).ilog2();

        ans.push((l, l + (1 << i)));

        l += 1 << i;
    }

    println!("{}", ans.len());
    for (x, y) in ans {
        println!("{x} {y}");
    }
}
