#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t t1, t2, a1, a2, b1, b2;
    cin >> t1 >> t2 >> a1 >> a2 >> b1 >> b2;

    auto la = t1 * a1 + t2 * a2;
    auto lb = t1 * b1 + t2 * b2;

    if (la == lb) {
        cout << "infinity" << endl;
        return 0;
    }

    if (la < lb) {
        swap(a1, b1);
        swap(a2, b2);
        swap(la, lb);
    }

    if (a1 > b1) {
        cout << 0 << endl;
        return 0;
    }

    int64_t n = t1 * (b1 - a1) / (la - lb);
    while (n * (la - lb) < t1 * (b1 - a1)) {
        ++n;
    }

    if (n * (la - lb) == t1 * (b1 - a1)) {
        cout << 2 * n << endl;
    } else {
        cout << 2 * n - 1 << endl;
    }


}