#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n, d;
    cin >> n >> d;

    int64_t ans = 0;
    vector<vector<int64_t>> v(n, vector<int64_t>(d));
    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, d)) {
            cin >> v[i][j];
        }

        for (auto j : irange(0L, i)) {
            int64_t dist2 = 0;
            for (auto k : irange(0L, d)) {
                dist2 += (v[i][k] - v[j][k]) * (v[i][k] - v[j][k]);
            }

            int64_t dist = sqrt(dist2);
            if (dist * dist == dist2 || (dist + 1) * (dist + 1) == dist2) {
                ++ans;
            }
        }
    }

    cout << ans << endl;
}
