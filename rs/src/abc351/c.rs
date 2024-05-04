use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let b = a.into_iter().fold(vec![], |mut b, x| {
        b.push(x);

        while b.len() >= 2 && b[b.len() - 2] == b[b.len() - 1] {
            let y = b.pop().unwrap();
            b.pop();

            b.push(y + 1);
        }

        b
    });

    println!("{}", b.len());
}
