use std::cmp::{max, min};

use proconio::input_interactive;
use rustc_hash::FxHashMap;

fn main() {
    input_interactive! {
        n: usize, l: usize, r: usize,
    };

    let r = r + 1;

    let mut memo = FxHashMap::default();
    let q = count(l, r, 0, 1 << n, &mut memo);
    eprintln!("#{q}");

    let ans = calc(l, r, 0, 1 << n, &memo);

    println!("! {ans}");
}

fn count(
    l: usize,
    r: usize,
    b: usize,
    e: usize,
    memo: &mut FxHashMap<(usize, usize, usize, usize), usize>,
) -> usize {
    assert!(b <= l);
    assert!(l <= r);
    assert!(r <= e);

    if let Some(&a) = memo.get(&(l, r, b, e)) {
        return a;
    }

    let a = {
        let m = (b + e) / 2;

        if r <= b || e <= l || l == r {
            0
        } else if l == b && r == e {
            1
        } else if m <= l {
            count(l, r, m, e, memo)
        } else if r <= m {
            count(l, r, b, m, memo)
        } else {
            // ばらばらに選ぶ場合
            let c0 =
                count(max(l, b), min(r, m), b, m, memo) + count(max(l, m), min(r, e), m, e, memo);

            // 全体を選ぶ場合
            let c1 = 1 + count(b, l, b, m, memo) + count(r, e, m, e, memo);

            min(c0, c1)
        }
    };

    memo.insert((l, r, b, e), a);

    a
}

fn calc(
    l: usize,
    r: usize,
    b: usize,
    e: usize,
    memo: &FxHashMap<(usize, usize, usize, usize), usize>,
) -> usize {
    assert!(b <= l);
    assert!(l <= r);
    assert!(r <= e);

    if r <= b || e <= l || l == r {
        return 0;
    }

    if l == b && r == e {
        let i = (e - b).ilog2() as usize;
        println!("? {} {}", i, b / (1 << i));
        input_interactive!(x: usize);
        return x;
    }

    let m = (b + e) / 2;

    if m <= l {
        calc(l, r, m, e, memo)
    } else if r <= m {
        calc(l, r, b, m, memo)
    } else {
        if memo[&(max(l, b), min(r, m), b, m)] + memo[&(max(l, m), min(r, e), m, e)]
            <= 1 + memo[&(b, l, b, m)] + memo[&(r, e, m, e)]
        {
            // ばらばらに選ぶ
            (calc(max(l, b), min(r, m), b, m, memo) + calc(max(l, m), min(r, e), m, e, memo)) % 100
        } else {
            // 全体を選ぶ
            (calc(b, e, b, e, memo) + 200 - calc(b, l, b, m, memo) - calc(r, e, m, e, memo)) % 100
        }
    }
}
