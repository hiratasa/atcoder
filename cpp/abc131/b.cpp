#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, l;
    cin >> n >> l;

    int64_t s = 0;
    for (auto i : irange(0L, n)) {
        s += l + i;
    }

    if (l * (l + n - 1) <= 0) {
        // NOP
    } else if (abs(l) < abs(l + n - 1)) {
        s -= l;
    } else {
        s -= l + n - 1;
    }

    cout << s << endl;
}