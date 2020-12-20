#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s;
    cin >> s;

    constexpr auto M = 998244353L;

    int64_t n = s.size();

    vector<int64_t> dp(2 * n);
    dp[0] = 1;

    for (auto a :
         s | transformed([](char c) { return c - '0'; }) | indexed(0)) {
        auto i = a.index();
        auto b = a.value();

        vector<int64_t> next(dp.size());
        for (auto j : irange(0L, 2 * n)) {
            int64_t remain = i + 2;
            int64_t blue = j + b;
            int64_t red = remain - blue;

            if (blue == 0) {
                next[0] += dp[j];
            } else if (red == 0) {
                next[blue - 1] += dp[j];
            } else {
                next[blue] += dp[j];
                next[blue - 1] += dp[j];
            }

            next[blue] %= M;
            next[blue - 1] %= M;
        }

        dp = std::move(next);
    }

    for (auto i : irange(0L, n)) {
        vector<int64_t> next(dp.size());
        for (auto j : irange(0L, 2 * n)) {
            int64_t remain = n - i;
            int64_t blue = j;
            int64_t red = remain - blue;

            if (blue == 0) {
                next[0] += dp[j];
            } else if (red == 0) {
                next[blue - 1] += dp[j];
            } else {
                next[blue] += dp[j];
                next[blue - 1] += dp[j];
            }

            next[blue] %= M;
            next[blue - 1] %= M;
        }

        dp = std::move(next);
    }

    std::cout << dp[0] << std::endl;
}