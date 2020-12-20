#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t a, b, c, d, e, f, g;
    cin >> a >> b >> c >> d >> e >> f >> g;

    auto t = a / 2 * 2 + d / 2 * 2 + e / 2 * 2;
    auto r = a % 2 + d % 2 + e % 2;
    if (r == 2 && a > 0 && d > 0 && e > 0) {
        t += 1;
    } else if (r == 3) {
        t += 3;
    }

    cout << b + t << endl;
}