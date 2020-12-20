#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    // y, x
    // +, -
    array<unordered_map<int64_t, array<vector<int64_t>, 4>>, 2> vals0;

    // x - y, x + y
    // up, right, bottom, left
    array<unordered_map<int64_t, array<vector<pair<int64_t, int64_t>>, 4>>, 2>
            vals1;
    for (auto _ : irange(0L, n)) {
        int64_t x, y;
        char u;
        cin >> x >> y >> u;

        int64_t dir;
        switch (u) {
            case 'U':
                dir = 0;
                break;
            case 'R':
                dir = 1;
                break;
            case 'D':
                dir = 2;
                break;
            case 'L':
                dir = 3;
                break;
            default:
                assert(false);
        }

        if (dir == 0 || dir == 2) {
            vals0[0][x][dir / 2].emplace_back(y);
        } else {
            vals0[1][y][dir / 2].emplace_back(x);
        }

        vals1[0][x - y][dir].emplace_back(x, y);
        vals1[1][x + y][dir].emplace_back(x, y);
    }

    for (auto i : irange(0L, 2L)) {
        for (auto&& kv : vals0[i]) {
            for (auto dir : irange(0L, 2L))
                sort(kv.second[dir].begin(), kv.second[dir].end());
        }
    }

    for (auto i : irange(0L, 2L)) {
        for (auto&& kv : vals1[i]) {
            for (auto dir : irange(0L, 4L))
                sort(kv.second[dir].begin(), kv.second[dir].end());
        }
    }

    int64_t ans = numeric_limits<int64_t>::max();

    for (auto i : irange(0L, 2L)) {
        for (const auto& kv : vals0[i]) {
            for (const auto& p : kv.second[0]) {
                auto it = lower_bound(kv.second[1].begin(), kv.second[1].end(),
                                      p);

                if (it != kv.second[1].end()) {
                    ans = min(ans, *it - p) * 5;
                }
            }
        }
    }

    // 0, 0, 3
    // 0, 1, 2
    // 1, 1, 0
    // 1, 2, 3

    for (auto i : irange(0L, 2L)) {
        for (const auto& kv : vals1[i]) {
            for (auto d : irange(0L + i, 2L + i)) {
                int64_t d2 = (i == 0 ? 3 - d : (5 - d) % 4);

                for (const auto& p : kv.second[d]) {
                    auto it = lower_bound(kv.second[d2].begin(),
                                          kv.second[d2].end(), p);

                    if (it != kv.second[d2].end()) {
                        ans = min(ans, it->first - p.first) * 10;
                    }
                }
            }
        }
    }

    if (ans == numeric_limits<int64_t>::max()) {
        cout << "SAFE" << endl;
    } else {
        cout << ans << endl;
    }
}