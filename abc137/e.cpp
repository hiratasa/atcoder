#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, m, p;
    cin >> n >> m >> p;

    vector<pair<pair<int64_t, int64_t>, int64_t>> l(m);
    for (auto&& ll : l) {
        cin >> ll.first.first >> ll.first.second >> ll.second;
        --ll.first.first;
        --ll.first.second;
    }

    vector<int64_t> points(n, numeric_limits<int64_t>::min());
    points[0] = 0;
    for (auto i : irange(0L, 2 * m + 1)) {
        for (const auto& ll : l) {
            if (points[ll.first.first] == numeric_limits<int64_t>::min()) {
                continue;
            }

            auto new_point = points[ll.first.first] + ll.second - p;
            if (new_point > points[ll.first.second]) {
                points[ll.first.second] = new_point;

                if (i >= m && ll.first.second == n - 1) {
                    cout << -1 << endl;
                    return 0;
                }
            }
        }
    }

    cout << max(points[n - 1], 0L) << endl;
}