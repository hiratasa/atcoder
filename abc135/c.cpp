#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n + 1);
    for (auto&& aa : a) {
        cin >> aa;
    }

    vector<int64_t> b(n);
    for (auto&& bb : b) {
        cin >> bb;
    }

    int64_t ans = 0;
    int64_t rem = 0;
    for (auto i : irange(0L, n)) {
        auto k = min(a[i], b[i]);
        ans += k;

        auto k2 = min(a[i + 1], b[i] - k);
        ans += k2;
        a[i + 1] -= k2;
    }

    cout << ans << endl;
}