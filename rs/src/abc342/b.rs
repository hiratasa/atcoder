use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: usize,
        ab: [(usize, usize); q],
    };

    ab.into_iter()
        .map(|(a, b)| {
            let i = p.iter().position(|&x| x == a).unwrap();
            let j = p.iter().position(|&x| x == b).unwrap();

            if i < j { a } else { b }
        })
        .for_each(|ans| {
            println!("{ans}");
        });
}
