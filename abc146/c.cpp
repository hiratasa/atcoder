#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t a, b, x;
    cin >> a >> b >> x;

    if (a * 1000000000L + b * 10L <= x) {
        cout << 1000000000L << endl;
        return 0;
    }

    int64_t cur = 1000000000L;
    for (auto d : irange(1L, 10L) | reversed) {
        cur /= 10;
        auto tmp = x - b * d;
        if (tmp <= 0) {
            continue;
        }

        auto lim = 10L * cur - 1L;
        int64_t n = min(lim, tmp / a);

        if (n < cur) {
            continue;
        }

        cout << n << endl;
        return 0;
    }

    cout << 0 << endl;
    return 0;
}