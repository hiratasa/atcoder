#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    unordered_map<int64_t, int64_t> x;
    for (auto _ : irange(0L, n)) {
        int64_t xx;
        cin >> xx;
        x[xx]++;
    }

    unordered_map<int64_t, pair<int64_t, int64_t>> y;
    for (const auto& kv : x) {
        auto xx = kv.first;
        y[xx % m].first += kv.second;
        y[xx % m].second += kv.second / 2;
    }

    int64_t ans = 0;
    for (auto i : irange(1L, (m + 1) / 2)) {
        auto mi = min(y[i], y[m - i]);
        auto ma = max(y[i], y[m - i]);

        ans += mi.first + min((ma.first - mi.first) / 2, ma.second);
    }

    ans += y[0].first / 2;
    if (m % 2 == 0) {
        ans += y[m / 2].first / 2;
    }

    cout << ans << endl;
}