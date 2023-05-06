#include <bits/stdc++.h>

#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    array<array<int64_t, 4>, 2> a;
    for (auto i : irange(0, 3)) {
        cin >> a[0][i];
    }
    for (auto i : irange(0, 3)) {
        cin >> a[1][i];
    }
    a[0][3] = a[1][3] = 1;

    int64_t m = n;
    int64_t s = 0, t = 1;
    for (auto _ : irange(0L, 2L)) {
        vector<int64_t> dp(m + 1, numeric_limits<int64_t>::min());
        dp[0] = 0;

        for (auto i : irange(0L, 4L)) {
            for (auto j : irange(0L, max(0L, m - a[s][i] + 1))) {
                dp[j + a[s][i]] = max(dp[j + a[s][i]], dp[j] + a[t][i]);
            }
        }

        m = dp[m];
        swap(s, t);
    }

    cout << m << endl;
}
