#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n, t;
    cin >> n >> t;

    vector<pair<int64_t, int64_t>> p(n);
    for (auto&& pp : p) {
        cin >> pp.first >> pp.second;
    }

    sort(p.rbegin(), p.rend());

    vector<int64_t> dp(t);
    for (auto pp : p) {
        for (auto i : irange(1L, t) | reversed) {
            if (i - pp.first >= 0) {
                dp[i] = max(dp[i - pp.first] + pp.second, dp[i]);
            }
        }
        dp[0] = max(pp.second, dp[0]);
    }

    cout << *max_element(dp.begin(), dp.end()) << endl;
}