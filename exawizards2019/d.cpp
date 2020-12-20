#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, x;
    cin >> n >> x;

    vector<int64_t> s(n);
    for (auto&& ss : s) {
        cin >> ss;
    }

    sort(s.begin(), s.end());

    constexpr auto M = 1000000007L;
    vector<vector<int64_t>> dp(n, vector<int64_t>(x + 1));

    for (auto j : irange(0L, x + 1)) {
        dp[0][j] = j % s[0];
    }
    for (auto i : irange(1L, n)) {
        for (auto j : irange(0L, x + 1)) {
            dp[i][j] = (dp[i - 1][j % s[i]] + dp[i - 1][j] * i % M) % M;
        }
    }

    cout << dp[n - 1][x] << endl;
}