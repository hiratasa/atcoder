#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, C;
    cin >> n >> C;

    vector<vector<int64_t>> d(C, vector<int64_t>(C));
    for (auto i : irange(0L, C)) {
        for (auto j : irange(0L, C)) {
            cin >> d[i][j];
        }
    }

    array<vector<int64_t>, 3> c{vector<int64_t>(C), vector<int64_t>(C),
                                vector<int64_t>(C)};
    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, n)) {
            int64_t cc;
            cin >> cc;
            --cc;

            ++c[(i + j) % 3][cc];
        }
    }

    array<vector<int64_t>, 3> dis{vector<int64_t>(C), vector<int64_t>(C),
                                  vector<int64_t>(C)};
    for (auto i : irange(0L, 3L)) {
        for (auto j : irange(0L, C)) {
            for (auto k : irange(0L, C)) {
                dis[i][j] += d[k][j] * c[i][k];
            }
        }
    }

    int64_t ans = numeric_limits<int64_t>::max();
    for (auto c0 : irange(0L, C)) {
        for (auto c1 : irange(0L, C)) {
            if (c0 == c1) {
                continue;
            }
            for (auto c2 : irange(0L, C)) {
                if (c0 == c2 || c1 == c2) {
                    continue;
                }

                auto t = dis[0L][c0] + dis[1L][c1] + dis[2L][c2];
                ans = min(t, ans);
            }
        }
    }

    cout << ans << endl;
}