#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    int64_t ans = 0;
    for (int64_t r = 1; r * (r + 1) < n; ++r) {
        if (n % r > 0) {
            continue;
        }

        int64_t m = n / r - 1;
        ans += m;
    }

    cout << ans << endl;
}