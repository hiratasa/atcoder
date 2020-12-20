#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, a, b;
    cin >> n >> a >> b;

    vector<int64_t> v(n);
    for (auto&& vv : v) {
        cin >> vv;
    }

    vector<pair<int64_t, int64_t>> dp(n + 1);
    dp[0] = {0L, 1L};
    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, n) | reversed) {
            if (dp[j].second == 0L) {
                continue;
            }
            if (dp[j + 1].first < dp[j].first + v[i]) {
                dp[j + 1].first = dp[j].first + v[i];
                dp[j + 1].second = dp[j].second;
            } else if (dp[j + 1].first == dp[j].first + v[i]) {
                dp[j + 1].second += dp[j].second;
            }
        }
    }

    int64_t af = 0, as = 1;
    int64_t w = 0;
    for (auto i : irange(a, b + 1)) {
        if (af * i < as * dp[i].first) {
            af = dp[i].first;
            as = i;
            w = dp[i].second;
        } else if (af * i == as * dp[i].first) {
            w += dp[i].second;
        }
    }

    cout << setprecision(20) << af / (double)as << endl;
    cout << w << endl;
}