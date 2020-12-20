#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t m, d;
    cin >> m >> d;

    if (21 >= d) {
        cout << 0 << endl;
        return 0;
    }

    int64_t ans = 0;
    for (auto mm : irange(1L, m + 1)) {
        for (auto day : irange(22L, d + 1)) {
            if (day % 10 < 2) {
                continue;
            }

            if (mm == (day / 10) * (day % 10)) {
                ++ans;
            }
        }
    }

    cout << ans << endl;
}