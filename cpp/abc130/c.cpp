#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    constexpr auto EPS = 1e-12;

    double w, h, x, y;
    cin >> w >> h >> x >> y;

/*
    if (y * w + h * x > w * h) {
        if (y * w - h * x > 0) {
            y = h - y;
        } else {
            swap(w, h);
            swap(x, y);
            y = h - y;
        }
    } else {
        if (y * w - h * x > 0) {
            swap(w, h);
            swap(x, y);
        } else {
            // NOP
        }
    }
    */

    cout << fixed;
    cout << setprecision(12) << w * h / 2.0;
    cout << " ";

    if (abs(x - w/2) < EPS && abs(y - h/2) < EPS) {
        cout << 1 << endl;
    } else {
        cout << 0 << endl;
    }
}