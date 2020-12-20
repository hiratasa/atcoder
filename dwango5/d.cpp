#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, d;
    cin >> n >> d;

    vector<vector<int>> count(d, vector<int>(d));
    vector<pair<int64_t, int64_t>> xy(n);
    for(auto&& p : xy) {
        cin >> p.first >> p.second;
        ++count[p.first % d][p.second % d];
    }

    int64_t a = 0;
    for (auto i : irange(0L, d)) {
        for (auto j : irange(0L, d)) {
            auto c = count[i][j];
            auto s = int64_t(sqrt(c));
            if (s * s < c) {
                ++s;
            }
            a = max(a, s - 1);
        }
    }

    vector<vector<int64_t>> num_inner(2 * d + 1, vector<int64_t>(2 * d + 1));
    vector<vector<int64_t>> num_outer(2 * d + 1, vector<int64_t>(2 * d + 1));
    for (auto i : irange(0L, 2 * d)) {
        for (auto j : irange(0L, 2 * d)) {
            bool in_inner = (count[i % d][j % d] - a * (a + 1) > 0);
            bool in_outer = (count[i % d][j % d] - a * a > 0);
            num_inner[i + 1][j + 1] = num_inner[i + 1][j] + num_inner[i][j + 1] - num_inner[i][j] + in_inner;
            num_outer[i + 1][j + 1] = num_outer[i + 1][j] + num_outer[i][j + 1] - num_outer[i][j] + in_outer;
        }
    }

    int64_t ans = numeric_limits<int64_t>::max();
    for (auto x : irange(0L, d)) {
        for (auto y : irange(0L, d)) {
            auto total_inner = num_inner[x + d][y + d] - num_inner[x + d][y] - num_inner[x][y + d];
            auto total_outer = num_outer[x + d][y + d];

            const auto& range = irange(0L, d);
            auto it = partition_point(range.begin(), range.end(), [&](int64_t b) {
                auto e = b + 1;
                auto ni = num_inner[x + e][y + e] - num_inner[x + e][y] - num_inner[x][y + e];
                if (ni < total_inner) {
                    return true;
                }

                auto no = num_outer[x + d][y + e] + num_outer[x + e][y + d] - num_outer[x + e][y + e];
                if (no < total_outer) {
                    return true;
                }

                return false;
            });

            ans = min(ans, *it);
        }
    }

    ans += a * d;

    cout << ans << endl;
}