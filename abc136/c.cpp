#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    int64_t prev = -1;
    for (auto i : irange(0L, n)) {
        int64_t h;
        cin >> h;

        if (h < prev) {
            cout << "No" << endl;
            return 0;
        } else if (h == prev) {
            continue;
        } else {
            prev = h - 1;
        }
    }

    cout << "Yes" << endl;
}