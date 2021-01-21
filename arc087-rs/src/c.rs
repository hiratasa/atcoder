use itertools::*;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        l: usize,
        s: [Chars; n]
    }

    let mut s = s;
    s.sort();
    let s = s;

    let common = s
        .iter()
        .tuple_windows()
        .map(|(s0, s1)| {
            izip!(s0.iter().copied(), s1.iter().copied())
                .take_while(|&(c0, c1)| c0 == c1)
                .count()
        })
        .collect_vec();

    let mut current = 0;
    let mut next_pos = 0;
    let mut next_c = '0';

    let grundy = |d: usize| 1 << (l - d + 1).trailing_zeros();

    let mut g = 0;

    while current < n {
        if next_pos == s[current].len() {
            while next_pos > 0 && s[current][next_pos - 1] == '1' {
                next_pos -= 1;
            }

            if next_pos == 0 {
                assert!(s[current][0] == '1');
                assert!(current == n - 1);
                break;
            }

            assert!(s[current][next_pos - 1] == '0');
            next_pos -= 1;
            next_c = '1';
        } else if next_c < s[current][next_pos] {
            // s[0..next_pos] + '0' does not exist in the set.
            g ^= grundy(next_pos + 1);
            next_c = '1';
        } else if next_c == s[current][next_pos] {
            next_pos += 1;
            next_c = '0';
        } else if next_c > s[current][next_pos] {
            if current + 1 < n && next_pos == common[current] {
                assert!(s[current + 1][next_pos] == '1');
                current += 1;
            } else {
                assert!(current == n - 1 || next_pos > common[current]);

                // s[0..next_pos] + '1' does not exist in the set.
                g ^= grundy(next_pos + 1);

                if next_pos == 0 {
                    break;
                }

                while next_pos > 0 && s[current][next_pos - 1] == '1' {
                    next_pos -= 1;
                }

                if next_pos == 0 {
                    assert!(s[current][0] == '1');
                    assert!(current == n - 1);
                    break;
                }

                assert!(s[current][next_pos - 1] == '0');
                assert!(current == n - 1 || next_pos > common[current]);
                next_pos -= 1;
                next_c = '1';
            }
        } else {
            unreachable!();
        }
    }

    if g == 0 {
        println!("Bob");
    } else {
        println!("Alice");
    }
}
