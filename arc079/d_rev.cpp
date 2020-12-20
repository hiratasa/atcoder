#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t k;
    cin >> k;

    auto m = k / 50;
    auto r = k % 50;

    cout << 50 << endl;
    const auto* delim = "";
    for (auto i : irange(0L, r)) {
        cout << delim << 49 + m + 1 + 50 - r;
        delim = " ";
    }

    for (auto i : irange(r, 50L)) {
        cout << delim << 49 + m - r;
        delim = " ";
    }
}