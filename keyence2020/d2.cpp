#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n), b(n);
    for (auto&& aa : a) {
        cin >> aa;
    }
    for (auto&& bb : b) {
        cin >> bb;
    }

    vector<vector<int64_t>> dp(
            1uL << n, vector<int64_t>(n, numeric_limits<int64_t>::max()));
    for (auto k : irange(0uL, 1uL << n)) {
        bitset<18> bs(k);

        for (auto i : irange(0L, n)) {
            if (!bs[i]) {
                continue;
            }

            auto v = (bs.count() - 1 - i + 2 * n) % 2 == 0 ? a[i] : b[i];

            auto prev = (bs ^ decltype(bs)(1uL << i));
            if (prev.none()) {
                dp[k][i] = 0;
                continue;
            }
            for (auto j : irange(0L, n)) {
                if (dp[prev.to_ulong()][j] == numeric_limits<int64_t>::max()) {
                    continue;
                }

                auto v2 = (prev.count() - 1 - j + 2 * n) % 2 == 0 ? a[j] : b[j];

                if (v < v2) {
                    continue;
                }

                int64_t m = dp[prev.to_ulong()][j];
                for (auto l : irange(i + 1, n)) {
                    if (bs[l]) {
                        ++m;
                    }
                }

                if (m < dp[k][i]) {
                    dp[k][i] = m;
                }
            }
        }
    }

    auto m = *min_element(dp.back().begin(), dp.back().end());

    if (m == numeric_limits<int64_t>::max()) {
        cout << -1 << endl;
    } else {
        cout << m << endl;
    }
}