#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t s;
    cin >> s;

    int64_t a = sqrt(s);
    if (a * a > s) {
        --a;
    }
    int64_t t = (a - 1) * (a + 1);

    int64_t y0, x1, x2, y2;
    if (a == 1000000000) {
        y0 = 0;
        x1 = 1000000000;
        x2 = 0;
        y2 = 1000000000;
    } else if (s - t <= 1000000000) {
        y0 = 1;
        x2 = s - t;
        y2 = a;
        x1 = a + 1;
    } else {
        auto u = a * (a + 1);
        y0 = 1;
        x2 = s - u;
        y2 = a + 1;
        x1 = a + 1;
    }

    cerr << (x1 * y2 - y0 * x1 + x2 * y0) << endl;
    cout << 0 << " " << y0 << " " << x1 << " " << 0 << " " << x2 << " " << y2
         << endl;
}