#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct Vector(f64, f64);

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector(self * rhs.0, self * rhs.1)
    }
}

impl Vector {
    pub const EPS: f64 = 1e-10;

    pub fn dot(self, rhs: Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }

    pub fn cross(self, rhs: Self) -> f64 {
        self.0 * rhs.1 - rhs.0 * self.1
    }

    pub fn norm(self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }

    pub fn distance(self, rhs: Self) -> f64 {
        (self - rhs).norm()
    }

    pub fn is_zero(self) -> bool {
        self.norm() < Self::EPS
    }

    pub fn is_parallel(self, rhs: Self) -> bool {
        self.is_zero()
            || rhs.is_zero()
            || self.cross(rhs).abs() < Self::EPS * f64::max(self.norm(), rhs.norm())
    }

    pub fn normalize(self) -> Option<Self> {
        let n = self.norm();
        if n == 0.0 {
            None
        } else {
            Some(Vector(self.0 / n, self.1 / n))
        }
    }
}

type Point = Vector;

// line by 2 points
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct Line(Point, Point);

#[allow(dead_code)]
impl Line {
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self(Vector(x0, y0), Vector(x1, y1))
    }

    pub fn slope(self) -> Vector {
        self.1 - self.0
    }

    pub fn is_parallel(self, rhs: Self) -> bool {
        self.slope().is_parallel(rhs.slope())
    }

    pub fn contains(self, p: Vector) -> bool {
        (self.1 - p).is_parallel(self.0 - p)
    }

    pub fn intersection(self, rhs: Self) -> Option<Vector> {
        if self.is_parallel(rhs) {
            if rhs.contains(self.0) {
                Some(self.0)
            } else {
                None
            }
        } else {
            Some(
                self.0
                    + rhs.slope().cross(rhs.0 - self.0) / rhs.slope().cross(self.slope())
                        * self.slope(),
            )
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct LineSegment(Vector, Vector);

#[allow(dead_code)]
impl LineSegment {
    pub const EPS: f64 = Vector::EPS;

    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self(Vector(x0, y0), Vector(x1, y1))
    }

    pub fn as_line(self) -> Line {
        Line(self.0, self.1)
    }

    pub fn slope(self) -> Vector {
        self.as_line().slope()
    }

    pub fn is_parallel(self, rhs: Self) -> bool {
        self.as_line().is_parallel(rhs.as_line())
    }

    pub fn contains(self, p: Point) -> bool {
        if self.as_line().contains(p) {
            let pp0 = self.0 - p;
            let pp1 = self.1 - p;
            pp0.dot(pp1) < Self::EPS * f64::max(pp0.norm(), pp1.norm())
        } else {
            false
        }
    }

    pub fn has_intersection(self, rhs: Self) -> bool {
        // TODO: crossがゼロ近いときに誤差で死にそう
        self.contains(rhs.0)
            || self.contains(rhs.1)
            || rhs.contains(self.0)
            || rhs.contains(self.1)
            || (self.slope().cross(rhs.0 - self.0).signum()
                * self.slope().cross(rhs.1 - self.0).signum()
                < 0.0
                && rhs.slope().cross(self.0 - rhs.0).signum()
                    * rhs.slope().cross(self.1 - rhs.0).signum()
                    < 0.0)
    }

    pub fn intersection(self, rhs: Self) -> Option<Vector> {
        if self.is_parallel(rhs) {
            if self.as_line().intersection(rhs.as_line()).is_none() {
                // not on the same line
                None
            } else if self.contains(rhs.0) {
                Some(rhs.0)
            } else if self.contains(rhs.1) {
                Some(rhs.1)
            } else if rhs.contains(self.0) {
                Some(self.0)
            } else if rhs.contains(self.1) {
                Some(self.1)
            } else {
                // on the same line but no overlap
                None
            }
        } else {
            let p = self.as_line().intersection(rhs.as_line()).unwrap();
            if self.contains(p) && rhs.contains(p) {
                Some(p)
            } else {
                None
            }
        }
    }
}

#[test]
fn test_segment_intersection() {
    let l0 = LineSegment::new(839., -642., -862., 371.);
    let l1 = LineSegment::new(531., -275., 82., -276.);

    assert!(l0.intersection(l1).is_some());

    let l2 = LineSegment::new(-1000., 141., -17., -1000.);
    let l3 = LineSegment::new(-232., 340., -428., -283.);

    assert!(l2.intersection(l3).is_none(), "{:?}", l2.intersection(l3));

    let l4 = LineSegment::new(-1000., 755., -86., -1000.);
    let l5 = LineSegment::new(-824., 343., 610., -202.);

    assert!(l4.intersection(l5).is_some(), "{:?}", l4.intersection(l5));
}

#[test]
fn test_segment_has_intersection() {
    let l0 = LineSegment::new(
        889.0199839841143,
        -165.47807681122208,
        378.55578345685126,
        -396.5046925601339,
    );
    let l1 = LineSegment::new(
        -820.4173399653221,
        48.984930371843575,
        646.2384608504906,
        -843.2775752563061,
    );

    assert!(!l0.has_intersection(l1));
}

#[test]
fn test_segment_intersection_random() {
    use rand::SeedableRng;
    use rand::distr::Distribution;

    let mut rng = rand::rngs::SmallRng::from_os_rng();
    let dist = rand::distr::Uniform::new(-1000.0, 1000.0).unwrap();

    for _ in 0..100 {
        let x0 = dist.sample(&mut rng);
        let y0 = dist.sample(&mut rng);
        let x1 = dist.sample(&mut rng);
        let y1 = dist.sample(&mut rng);
        let x2 = dist.sample(&mut rng);
        let y2 = dist.sample(&mut rng);
        let x3 = dist.sample(&mut rng);
        let y3 = dist.sample(&mut rng);

        let l0 = LineSegment::new(x0, y0, x1, y1);
        let l1 = LineSegment::new(x2, y2, x3, y3);

        assert_eq!(
            l0.intersection(l1).is_some(),
            l0.has_intersection(l1),
            "{},{} {},{} {},{} {},{}",
            x0,
            y0,
            x1,
            y1,
            x2,
            y2,
            x3,
            y3
        );
    }
}
