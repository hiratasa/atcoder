#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    vector<pair<int64_t, int64_t>> p;
    for (auto i : irange(0L, n)) {
        int64_t x, y;
        cin >> x >> y;
        if (x == 0 && y == 0) {
            continue;
        }

        p.emplace_back(x, y);
    }

    sort(p.begin(), p.end(), [](const auto& pp0, const auto& pp1) {
        if (pp0.first * pp1.first < 0) {
            return pp0.first > pp1.first;
        }

        return atan(pp0.second / (double)pp0.first) <
               atan(pp1.second / (double)pp1.first);
    });

    int64_t st = 0, ed = 1;
    while (false) {
        bool tmp = ((p[ed].first * p[st].first > 0)
                            ? (p[ed].first > -p[st].first)
                            : (atan(p[ed].second / (double)p[ed].first) <
                               atan(p[st].second / (double)p[st].first)));

        if (tmp) {
        }
    }
}