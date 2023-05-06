#include <bits/stdc++.h>

using namespace std;

template <typename T = long double>
using Point = complex<T>;

template <typename T>
T distance(const Point<T>& lhs, Point<T>& rhs) {
    return abs(lhs - rhs);
}

template <typename T>
T arg(const Point<T>& lhs, const Point<T>& rhs) {
    return arg(rhs / lhs);
}

template <typename T>
T dot(const Point<T>& lhs, const Point<T>& rhs) {
    return (lhs * conj(rhs)).real();
}

template <typename T>
T cross(const Point<T>& lhs, const Point<T>& rhs) {
    return (conj(lhs) * rhs).imag();
}

template <typename T>
T area(const Point<T>& a, const Point<T>& b, const Point<T>& c) {
    return cross(b - a, c - a) / 2;
}

// 二点指定の直線
template <typename T = long double>
struct LineBy2Point {
    Point<T> slope() const { return p2 - p1; }

    Point<T> p1;
    Point<T> p2;
};

constexpr auto EPS = 1e-10;

template <typename T>
bool is_parallel(const LineBy2Point<T>& l1, const LineBy2Point<T>& l2) {
    return abs(cross(l1.slope(), l2.slope())) < EPS;
}

template <typename T>
pair<bool, Point<T>> line_intersection(const LineBy2Point<T>& l1,
                                       const LineBy2Point<T>& l2) {
    // parallel
    if (abs(cross(l1.slope(), l2.slope())) < EPS) {
        if (abs(cross(l2.p2 - l1.p1, l2.p1 - l1.p1)) < EPS) {
            return make_pair(true, l1.p1);
        } else {
            return make_pair(false, Point<T>());
        }
    }

    return make_pair(true, l1.p1 + cross(l2.slope(), l2.p1 - l1.p1) /
                                           cross(l2.slope(), l1.slope()) *
                                           l1.slope());
}

template <typename T>
pair<bool, Point<T>> segment_intersection(const LineBy2Point<T>& l1,
                                          const LineBy2Point<T>& l2) {
    if (is_parallel(l1, l2)) {
        if (dot(l1.p1 - l2.p1, l1.p2 - l2.p1) <= EPS) {
            return make_pair(true, l2.p1);
        } else if (dot(l1.p1 - l2.p2, l1.p2 - l2.p2) <= EPS) {
            return make_pair(true, l2.p2);
        } else if (dot(l2.p1 - l1.p1, l2.p2 - l1.p1) <= EPS) {
            return make_pair(true, l1.p1);
        } else if (dot(l2.p1 - l1.p2, l2.p2 - l1.p2) <= EPS) {
            return make_pair(true, l1.p2);
        } else {
            return make_pair(false, Point<T>());
        }
    }

    auto tmp = line_intersection(l1, l2).second;
    if (dot(l1.p1 - tmp, l1.p2 - tmp) <= EPS &&
        dot(l2.p1 - tmp, l2.p2 - tmp) <= EPS) {
        return make_pair(true, tmp);
    } else {
        return make_pair(false, Point<T>());
    }
}

template <typename T>
bool on_segment(const Point<T>& p, const LineBy2Point<T>& l) {
    return abs(cross(l.p1 - p, l.p2 - p)) <= EPS &&
           dot(l.p1 - p, l.p2 - p) <= EPS;
}

main() {
    Point<int64_t> p1{1, 2}, p2{3, 4}, p3{4, 6};
    assert(p1 + p2 == p3);
    assert(dot(p1, p2) == 11);
    assert(cross(p1, p2) == -2);

    Point<> q1{1, 2}, q2{3, 4};
    assert(area({0, 0}, q1, q2) == -1);

    LineBy2Point<> l1{{1, 3}, {3, 6}}, l2{{0, 2}, {2, 5}}, l3{{0, 3}, {2, 4}},
            l4{{2, 4}, {4, 5}};
    assert(is_parallel(l1, l2));
    assert(!is_parallel(l1, l3));

    assert(line_intersection(l1, l3) == make_pair(true, Point<>{1.5, 3.75}));
    assert(line_intersection(l1, l4) == make_pair(true, Point<>{1.5, 3.75}));
    assert(segment_intersection(l1, l3) == make_pair(true, Point<>{1.5, 3.75}));
    assert(segment_intersection(l1, l4) == make_pair(false, Point<>()));
}