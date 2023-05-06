#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<vector<int64_t>> p(8, vector<int64_t>(n));
    for (auto i : irange(0L, n)) {
        std::array<int64_t, 3> x;
        cin >> x[0] >> x[1] >> x[2];

        for (auto s : irange(0uL, 8uL)) {
            for (auto j : irange(0L, 3L)) {
                p[s][i] += (((s >> j) & 1uL) > 0) ? x[j] : -x[j];
            }
        }
    }

    int64_t ans = numeric_limits<int64_t>::min();
    for (auto s : irange(0uL, 8uL)) {
        sort(p[s].rbegin(), p[s].rend());

        int64_t pp = 0;
        for (auto i : irange(0L, m)) {
            pp += p[s][i];
        }

        ans = max(ans, pp);
    }

    cout << ans << endl;
}