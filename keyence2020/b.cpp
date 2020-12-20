#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<pair<int64_t, int64_t>> p(n);
    for (auto i : irange(0L, n)) {
        int64_t x, l;
        cin >> x >> l;

        p[i] = {x + l, x - l};
    }

    sort(p.begin(), p.end());

    vector<int64_t> dp(n + 1);
    for (auto i : irange(0L, n)) {
        auto pp = p[i];
        auto s = pp.second;

        auto idx =
                upper_bound(p.begin(), p.end(), s,
                            [&](int64_t s, const pair<int64_t, int64_t>& pp) {
                                return s < pp.first;
                            }) -
                p.begin() - 1;

        if (idx >= 0) {
            dp[i + 1] = max(dp[i], dp[idx + 1] + 1);
        } else {
            dp[i + 1] = max(dp[i], 1L);
        }
    }

    cout << dp.back() << endl;
}