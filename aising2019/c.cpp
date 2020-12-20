#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int64_t dfs(const vector<vector<bool>>& is_white, vector<vector<pair<int64_t, int64_t>>>* visited, int64_t sx, int64_t sy, int64_t x, int64_t y) {
    auto self_is_white = is_white[x][y];

    if ((*visited)[x][y].first >= 0) {
        return 0;
    }
    (*visited)[x][y] = make_pair(sx, sy);

    int64_t ret = 0;
    if (self_is_white) {
        ++ret;
    }

    if (x > 0) {
        if (is_white[x - 1][y] != self_is_white) {
            ret += dfs(is_white, visited, sx, sy, x - 1, y);
        }
    }
    if (x < is_white.size() - 1) {
        if (is_white[x + 1][y] != self_is_white) {
            ret += dfs(is_white, visited, sx, sy, x + 1, y);
        }
    }
    if (y > 0) {
        if (is_white[x][y - 1] != self_is_white) {
            ret += dfs(is_white, visited, sx, sy, x, y - 1);
        }
    }
    if (y < is_white[0].size() - 1) {
        if (is_white[x][y + 1] != self_is_white) {
            ret += dfs(is_white, visited, sx, sy, x, y + 1);
        }
    }

    return ret;
}

main() {
    int64_t h, w;
    cin >> h >> w;

    vector<vector<bool>> is_white(h, vector<bool>(w));
    for (auto i : irange(0L, h)) {
        string s;
        cin >> s;
        for (auto j : irange(0L, w)) {
            is_white[i][j] = (s[j] != '#');
        }
    }

    int64_t ans = 0;
    vector<vector<int64_t>> cache(h, vector<int64_t>(w, 0));
    vector<vector<pair<int64_t, int64_t>>> visited(h, vector<pair<int64_t, int64_t>>(w, make_pair(-1L, -1L)));
    for (auto i : irange(0L, h)) {
        for (auto j : irange(0L, w)) {
            if (is_white[i][j]) {
                continue;
            }

            if (visited[i][j].first >= 0) {
                auto start = visited[i][j];
                ans += cache[start.first][start.second];
                // cerr << "cached: " << i << "," << j << ":" << cache[start.second][start.second] << " " << start.first << "," << start.second << endl;
            } else {
                auto ret = dfs(is_white, &visited, i, j, i, j);
                cache[i][j] = ret;
                ans += ret;
                // cerr << i << "," << j << ":" << ret << endl;
            }

        }
    }

    cout << ans << endl;
}