#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t x;
    cin >> x;

    int64_t n = x / 100;
    int64_t m = x % 100;

    int64_t k = 0;
    for (auto p : {5, 4, 3, 2, 1}) {
        k += m / p;
        m -= (m / p) * p;
    }

    if (k > n) {
        cout << 0 << endl;
    } else {
        cout << 1 << endl;
    }
}