#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    vector<unordered_map<int64_t, bool>> xy(n);
    for (auto i : irange(0L, n)) {
        int64_t a;
        cin >> a;

        for (auto j : irange(0L, a)) {
            int64_t x, y;
            cin >> x >> y;
            --x;
            xy[i][x] = y;
        }
    }

    uint64_t ans = 0;
    for (uint64_t b = 0; b < (1L << n); ++b) {
        bitset<15> bs(b);

        bool ok = true;
        for (auto i : irange(0L, n)) {
            if (!bs[i]) {
                continue;
            }
            for (const auto& kv : xy[i]) {
                if (bs[kv.first] != kv.second) {
                    ok = false;
                    break;
                }
            }
            if (!ok) {
                break;
            }
        }

        if (ok) {
            ans = max(ans, bs.count());
        }
    }

    cout << ans << endl;
}