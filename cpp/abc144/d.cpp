#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t a, b, x;
    cin >> a >> b >> x;

    auto t = 2 * (a * a * b - x) / (double)a / a / a;
    if (t * a >= b) {
        t = a * b * b / 2.0 / x;
    }

    cout << setprecision(10) << atan(t) / M_PI * 180.0 << endl;
}