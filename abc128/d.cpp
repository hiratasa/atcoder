#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, k;
    cin >> n >> k;

    vector<int64_t> v(n);
    for (auto&& vv : v) {
        cin >> vv;
    }

    int64_t ans = numeric_limits<int64_t>::min();
    for (auto l : irange(0L, n + 1)) {
        for (auto r : irange(0L, n + 1)) {
            if (l + r > n) {
                continue;
            }

            if (l + r > k) {
                continue;
            }

            vector<int64_t> items;
            items.insert(items.end(), v.begin(), v.begin() + l);
            items.insert(items.end(), v.end() - r, v.end());
            sort(items.begin(), items.end());

            int64_t s = 0;
            auto c = k - l - r;
            for (auto p : items) {
                if (p < 0 && c > 0) {
                    --c;
                    continue;
                }
                s += p;
            }

            ans = max(ans, s);
        }
    }

    cout << ans << endl;
}