#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h, w;
    cin >> h >> w;

    vector<vector<bool>> s(h + 2, vector<bool>(w + 2));
    for (auto i : irange(0L, h)) {
        string ss;
        cin >> ss;
        for (auto j : irange(0L, w)) {
            s[i + 1][j + 1] = (ss[j] == '.');
        }
    }

    int64_t m = 0;
    vector<int64_t> dx{0, 0, 1, -1};
    vector<int64_t> dy{1, -1, 0, 0};
    for (auto i : irange(1L, h + 1)) {
        for (auto j : irange(1L, w + 1)) {
            if (!s[i][j]) {
                continue;
            }

            vector<vector<int64_t>> dist(h + 2, vector<int64_t>(w + 2, -1L));
            queue<pair<int64_t, int64_t>> q;

            dist[i][j] = 0;
            q.emplace(i, j);
            while (!q.empty()) {
                auto now = q.front();
                q.pop();

                for (auto idx : irange(0L, 4L)) {
                    auto next = now;
                    next.first += dy[idx];
                    next.second += dx[idx];

                    if (!s[next.first][next.second]) {
                        continue;
                    }

                    if (dist[next.first][next.second] >= 0) {
                        continue;
                    }

                    dist[next.first][next.second] =
                            dist[now.first][now.second] + 1;
                    q.push(next);
                }
            }

            for (auto i2 : irange(1L, h + 1)) {
                for (auto j2 : irange(1L, w + 1)) {
                    m = max(m, dist[i2][j2]);
                }
            }
        }
    }

    cout << m << endl;
}