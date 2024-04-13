use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        a: [[i64; 3]; 3],
    };

    if calc_winner(&a, &mut [[None; 3]; 3], false, 0, &mut [0; 2]) {
        println!("Aoki");
    } else {
        println!("Takahashi");
    }
}

fn calc_winner(
    a: &[Vec<i64>],
    used: &mut [[Option<bool>; 3]; 3],
    turn: bool,
    idx: usize,
    points: &mut [i64],
) -> bool {
    for (i, j) in iproduct!(0..3, 0..3) {
        if used[i][j].is_some() {
            continue;
        }

        used[i][j] = Some(turn);
        points[turn as usize] += a[i][j];

        let mut win = (0..3).all(|ii| used[ii][j] == Some(turn))
            || (0..3).all(|jj| used[i][jj] == Some(turn))
            || (0..3).all(|ii| used[ii][ii] == Some(turn))
            || (0..3).all(|ii| used[ii][2 - ii] == Some(turn))
            || (idx == 8 && points[turn as usize] > points[(!turn) as usize]);

        if !win && idx + 1 < 9 {
            win = calc_winner(a, used, !turn, idx + 1, points) == turn;
        }

        points[turn as usize] -= a[i][j];
        used[i][j] = None;

        if win {
            return turn;
        }
    }

    !turn
}
