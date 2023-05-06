#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<int64_t> costs(m);
    vector<vector<int64_t>> idols(m, vector<int64_t>(n));

    for (auto i : irange(0L, m)) {
        int64_t c;
        cin >> c >> costs[i];
        idols[i].resize(c);
        for (auto j : irange(0L, c)) {
            int64_t id, p;
            cin >> id >> p;
            --id;
            idols[i][id] = p;
        }
    }

    vector<double> dp(1L << n);
    for (auto s : irange(1uL, 1uL << n)) {
        bitset<10> bs(s);

        dp[s] = 1L << 30;
        for (auto i : irange(0L, m)) {
            double c1 = 100 * costs[i];
            int64_t p = 0;
            for (auto j : irange(0L, n)) {
                if (bs[j]) {
                    c1 += dp[s & ~(1uL << j)] * idols[i][j];
                    p += idols[i][j];
                }
            }

            if (p == 0) {
                continue;
            }

            c1 /= p;

            dp[s] = min(dp[s], c1);
        }
    }

    cout << setprecision(20) << dp.back() << endl;
}