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

    vector<vector<bool>> maze(n + 2, vector<bool>(m + 2));
    int64_t sy, sx, gy, gx;
    for (auto i : irange(0L, n)) {
        string line;
        cin >> line;
        for (auto j : irange(0L, m)) {
            if (line[j] == 'W') {
                continue;
            }

            maze[i + 1][j + 1] = true;

            if (line[j] == 'S') {
                sy = i + 1;
                sx = j + 1;
            } else if (line[j] == 'G') {
                gy = i + 1;
                gx = j + 1;
            }
        }
    }

    queue<pair<int64_t, int64_t>> q;
    vector<vector<int64_t>> costs(n + 2, vector<int64_t>(m + 2, -1L));
    q.emplace(sy, sx);
    costs[sy][sx] = 0;
    while (!q.empty()) {
        auto [y, x] = q.front();
        q.pop();

        if (y == gy && x == gx) {
            cout << costs[gy][gx] << endl;
            return 0;
        }

        int64_t dy[] = {-1, 1, 0, 0};
        int64_t dx[] = {0, 0, -1, 1};
        for (auto i : irange(0L, 4L)) {
            if (maze[y + dy[i]][x + dx[i]] && costs[y + dy[i]][x + dx[i]] < 0) {
                costs[y + dy[i]][x + dx[i]] = costs[y][x] + 1;
                q.emplace(y + dy[i], x + dx[i]);
            }
        }
    }

    cout << -1 << endl;
}