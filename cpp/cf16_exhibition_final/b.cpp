#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

using Point = complex<double>;

istream& operator>>(istream& is, Point& p) {
    double x, y;
    is >> x >> y;
    p = Point(x, y);
    return is;
}

double dist(const Point& p1, const Point& p2) { return abs(p1 - p2); }

double calc(const double& a, const double& b, const double& c, double s) {
    // s = (2r+a)*r/2 + (b + c) * r / 2 + 2r * (2s/a - r) / 2
    //   = (a+b+c)*r/2 + 2sr/a
    // => r = s/[(a+b+c)/2+2s/a]

    return s / ((a + b + c) / 2. + 2 * s / a);
}

int main() {
    array<Point, 3> points;

    for (auto i : irange(0L, 3L)) {
        cin >> points[i];
    }

    array<double, 3> a;
    for (auto i : irange(0L, 3L)) {
        a[i] = dist(points[i], points[(i + 1) % 3]);
    }

    auto t = (a[0] + a[1] + a[2]) / 2.;
    double s = sqrt(t * (t - a[0]) * (t - a[1]) * (t - a[2]));

    cout << setprecision(20)
         << max({calc(a[0], a[1], a[2], s), calc(a[1], a[2], a[0], s),
                 calc(a[2], a[0], a[1], s)})
         << endl;
}