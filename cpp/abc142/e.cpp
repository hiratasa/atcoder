#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n, m;
    cin >> n >> m;

    vector<int64_t> a(m);
    vector<uint64_t> c(m);
    for (auto i : irange(0L, m)) {
        int64_t b;
        cin >> a[i] >> b;

        for (auto _ : irange(0L, b)) {
            int64_t cc;
            cin >> cc;
            --cc;
            c[i] |= (1uL << cc);
        }
    }

    vector<int64_t> dp(1uL << n, numeric_limits<int64_t>::max());
    dp[0] = 0;
    for (auto i : irange(0uL, 1uL << n)) {
        if (dp[i] == numeric_limits<int64_t>::max()) {
            continue;
        }
        for (auto key : irange(0L, m)) {
            auto next_key = (i | c[key]);
            dp[next_key] = min(dp[next_key], dp[i] + a[key]);
        }
    }

    if (dp[(1uL << n) - 1] == numeric_limits<int64_t>::max()) {
        cout << -1 << endl;
    } else {
        cout << dp[(1uL << n) - 1] << endl;
    }
}