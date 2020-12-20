#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h, w, k;
    cin >> h >> w >> k;

    int64_t x1, y1, x2, y2;
    cin >> x1 >> y1 >> x2 >> y2;

    vector ok(h + 2, vector(w + 2, false));
    for (auto i : irange(0L, h)) {
        string s;
        cin >> s;
        for (auto j : irange(0L, w)) {
            ok[i + 1][j + 1] = (s[j] == '.');
        }
    }

    array dx{-1L, 1L, 0L, 0L};
    array dy{0L, 0L, -1L, 1L};

    vector costs(h + 2, vector(w + 2, numeric_limits<int64_t>::max()));
    queue<std::tuple<int64_t, int64_t>> q;
    costs[x1][y1] = 0;
    q.emplace(x1, y1);

    while (!q.empty()) {
        auto [x, y] = q.front();
        q.pop();

        if (x == x2 && y == y2) {
            cout << costs[x][y] << endl;
            return 0;
        }

        auto nc = costs[x][y] + 1;
        for (auto d : irange(0L, 4L)) {
            for (auto i : irange(1L, k + 1)) {
                auto nx = x + i * dx[d];
                auto ny = y + i * dy[d];

                if (!ok[nx][ny]) {
                    break;
                }

                if (costs[nx][ny] < nc) {
                    break;
                }

                if (nc < costs[nx][ny]) {
                    costs[nx][ny] = nc;
                    q.emplace(nx, ny);
                }
            }
        }
    }

    cout << -1 << endl;
}