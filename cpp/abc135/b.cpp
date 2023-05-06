#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    int64_t c = 0;
    for (auto i : irange(0L, n)) {
        int64_t p;
        cin >> p;

        if (p != i + 1) {
            ++c;
        }
    }

    if (c <= 2) {
        cout << "YES" << endl;
    } else {
        cout << "NO" << endl;
    }
}