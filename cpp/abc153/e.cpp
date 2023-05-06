#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h, n;
    cin >> h >> n;

    vector<int64_t> dp(h + 1, numeric_limits<int32_t>::max());
    dp[0] = 0;
    for (auto i : irange(0L, n)) {
        int64_t a, b;
        cin >> a >> b;

        for (auto j : irange(0L, h)) {
            dp[min(j + a, h)] = min(dp[min(j + a, h)], dp[j] + b);
        }
    }

    cout << dp[h] << endl;
}