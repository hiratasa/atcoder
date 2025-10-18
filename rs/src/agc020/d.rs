#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_cols<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

fn print_chars(s: &str, pos: usize, len: usize, c: usize, d: usize) {
    assert!(c <= d);

    if pos + len <= c {
        return;
    }

    if d < pos {
        return;
    }

    let print_min_pos = max(c, pos);
    let print_max_pos = min(pos + len - 1, d);

    for _ in 0..print_max_pos - print_min_pos + 1 {
        print!("{}", s);
    }
}

fn solve_query2(a: usize, b: usize, c: usize, d: usize) {
    let len = a + b;

    let min_ab = min(a, b);
    let max_ab = max(a, b);
    let len_longest = (max_ab - 1) / (min_ab + 1) + 1;

    let mut va = a;
    let mut vb = b;
    let mut pos = 0;

    if pos <= d && va >= len_longest && (va - len_longest + 1) * len_longest >= vb {
        print_chars("A", pos, len_longest, c, d);
        pos += len_longest;
        va -= len_longest;
    }

    // len_longestの定義から.
    // 等号が成り立つときは、(va, vb) == (0, 0) まで回る
    assert!(vb * len_longest >= va);
    while pos <= d && va >= len_longest && (va - len_longest + 1) * len_longest >= vb - 1 && vb >= 1
    {
        print_chars("B", pos, 1, c, d);
        pos += 1;
        vb -= 1;
        print_chars("A", pos, len_longest, c, d);
        pos += len_longest;
        va -= len_longest;

        assert!(vb * len_longest - va >= 0)
    }

    assert!(vb * len_longest > va || (va == 0 && vb == 0));
    assert!(
        va < len_longest || (va - len_longest + 1) * len_longest < vb - 1 || (va == 0 && vb == 0)
    );

    if pos <= d && vb >= 2 && va > (vb - 2) / len_longest {
        print_chars("B", pos, 1, c, d);
        pos += 1;
        vb -= 1;

        // ↑で引いているのでここは-1
        let len_a = va - (vb - 1) / len_longest;
        assert!(len_a < len_longest);
        print_chars("A", pos, len_a, c, d);
        pos += len_a;
        va -= len_a;
        assert_eq!(va, (vb - 1) / len_longest);
    }

    assert!((va == 0 && vb == 0) || (vb == 1 && va < len_longest) || va * len_longest < vb);

    if pos <= d
    /* && k */
    {
        let len_b = vb - va * len_longest;
        assert!(len_b <= len_longest);
        print_chars("B", pos, vb - va * len_longest, c, d);
        pos += len_b;
        vb -= len_b;
    }
    assert_eq!(vb, va * len_longest);

    while pos <= d && va > 0 {
        print_chars("A", pos, 1, c, d);
        pos += 1;
        va -= 1;

        print_chars("B", pos, len_longest, c, d);
        pos += len_longest;
        vb -= len_longest;
    }

    // (a - len_longest * (n - 1)) >= b - (n - 1) かつ
    // ⇔ n <= (a - b) / (len_longest - 1) + 1
    // が成り立つ最大のnまでは A^{len_longest}B の繰り返し
    /*
    while pos <= d {
        if va * len_longest < vb {
            let next_len_b = min(len_longest, vb - (va * len_longest));
            print_chars("B", pos, next_len_b, c, d);
            pos += next_len_b;
            vb -= next_len_b;
        } else {
            (va - x) * len_longest >= vb
            va*len_longest - vb >= x * len_longest
            let next_len_a = min(len_longest, va - (vb - 1) / len_longest);
            print_chars("A", pos, next_len_a, c, d);
            pos += next_len_a;
            va -= next_len_a;
            if vb >= 1 {
                print_chars("B", pos, 1, c, d);
                pos += 1;
                vb -= 1;
            }
        }
    }
    */

    println!("");
}

fn main() {
    // 20:01:15
    let num_q = read::<usize>();
    for _ in 0..num_q {
        let query = read_cols::<usize>();
        // solve_query(query[0], query[1], query[2] - 1, query[3] - 1);
        solve_query2(query[0], query[1], query[2] - 1, query[3] - 1);
    }
}
