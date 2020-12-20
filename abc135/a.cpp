#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t a, b;
    cin >> a >> b;

    auto k = (a + b) / 2;

    if (abs(a - k) == abs(b - k)) {
        cout << k << endl;
    } else {
        cout << "IMPOSSIBLE" << endl;
    }
}