use proconio::input;

fn main() {
    input! {
        mut h: usize, mut w: usize, n: usize,
        a: [usize; n],
    };

    let mut hh = 0;
    let mut ww = 0;
    let mut t = vec![0; 26];
    for i in (0..=25).rev() {
        if hh == 0 {
            assert_eq!(ww, 0);
            let k = (h / (1 << i)) * (w / (1 << i));
            t[i] += k;
            if k > 0 {
                hh = h / (1 << i) * (1 << i);
                ww = w / (1 << i) * (1 << i);
            }
        } else {
            assert_ne!(ww, 0);

            {
                let k = ((h - hh) / (1 << i)) * (ww / (1 << i));
                t[i] += k;
                if k > 0 {
                    hh += (h - hh) / (1 << i) * (1 << i);
                }
            }

            {
                let k = ((w - ww) / (1 << i)) * (hh / (1 << i));
                t[i] += k;
                if k > 0 {
                    ww += (w - ww) / (1 << i) * (1 << i);
                }
            }
        }

        if hh == h {
            w -= ww;
            hh = 0;
            ww = 0;
        } else if ww == w {
            h -= hh;
            hh = 0;
            ww = 0;
        }
    }

    let freq = a.into_iter().fold(vec![0; 26], |mut freq, x| {
        freq[x] += 1;
        freq
    });

    eprintln!("{t:?}");

    for i in (0..=25).rev() {
        if t[i] < freq[i] {
            println!("No");
            return;
        }

        t[i] -= freq[i];

        if i > 0 {
            t[i - 1] += 4 * t[i];
        }
    }

    println!("Yes");
}
