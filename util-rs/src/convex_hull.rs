#[allow(dead_code)]
#[derive(Clone, Copy, PartialOrd, PartialEq)]
struct Point(f64, f64);

#[allow(dead_code)]
impl std::ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[allow(dead_code)]
impl std::ops::Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        *self - *rhs
    }
}

#[allow(dead_code)]
impl Point {
    fn dot(&self, rhs: Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }

    fn cross(&self, rhs: Self) -> f64 {
        self.0 * rhs.1 - rhs.0 * self.1
    }

    fn norm(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }

    fn normalize(&self) -> Option<Self> {
        let n = self.norm();
        if n == 0.0 {
            None
        } else {
            Some(Point(self.0 / n, self.1 / n))
        }
    }
}

#[allow(dead_code)]
fn convex_hull(points: &Vec<Point>) -> Vec<usize> {
    assert!(points.len() > 2);

    let idxs = {
        let mut idxs = (0..points.len()).collect::<Vec<_>>();
        idxs.sort_by(|&idx1, &idx2| points[idx1].partial_cmp(&points[idx2]).unwrap());
        idxs
    };

    let lower_ch =
        idxs.iter()
            .map(|&i| (i, &points[i]))
            .fold(vec![], |mut ch: Vec<usize>, (i, p)| {
                // 凸包の辺上のものも選んでいる
                // (辺上のものを選ばない場合はcross積が0のときにもpopする)
                while ch.len() >= 2
                    && (points[ch[ch.len() - 1]] - points[ch[ch.len() - 2]])
                        .cross(p - &points[ch[ch.len() - 2]])
                        < 0.0
                {
                    ch.pop();
                }

                ch.push(i);

                ch
            });
    let t = lower_ch.len();
    let mut ch = idxs.iter().rev().map(|&i| (i, &points[i])).skip(1).fold(
        lower_ch,
        |mut ch: Vec<usize>, (i, p)| {
            while ch.len() >= t + 1
                && (points[ch[ch.len() - 1]] - points[ch[ch.len() - 2]])
                    .cross(p - &points[ch[ch.len() - 2]])
                    < 0.0
            {
                ch.pop();
            }

            ch.push(i);

            ch
        },
    );
    ch.pop();
    ch
}
