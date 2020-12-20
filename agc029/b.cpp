#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int32_t n;
    cin >> n;
    unordered_map<int64_t, int32_t> a;
    vector<int64_t> s(n);
    for (auto i : irange(0, n)) {
        int64_t aa;
        cin >> aa;
        ++a[aa];
        s[i] = aa;
    }

    sort(s.begin(), s.end());

    int64_t ans = 0;
    for (int p = 30; p >= 1; --p) {
        auto m = (1L << p);
        for (auto aa : s) {
            if (aa > m / 2) break;

            if (aa * 2 == m) {
                int count = a[aa] / 2;
                ans += count;
                a[aa] -= 2 * count;
            } else {
                int count = min(a[aa], a[m - aa]);
                ans += count;
                a[aa] -= count;
                a[m - aa] -= count;
            }
        }
    }

    cout << ans << endl;
}