#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

bool dfs(const vector<vector<int64_t>>& adjs, vector<int64_t>& match,
         vector<bool>& used, int64_t v) {
    used[v] = true;

    for (auto u : adjs[v]) {
        auto w = match[u];
        if (w < 0 || (!used[w] && dfs(adjs, match, used, w))) {
            match[u] = v;
            match[v] = u;
            return true;
        }
    }

    return false;
}

int main() {
    int64_t m, n;
    cin >> m >> n;

    vector<vector<bool>> a(m, vector<bool>(n)), b(m, vector<bool>(n));
    for (auto i : irange(0L, m)) {
        for (auto j : irange(0L, n)) {
            int64_t x;
            cin >> x;
            a[i][j] = (x == 1);
        }
    }
    for (auto i : irange(0L, m)) {
        for (auto j : irange(0L, n)) {
            int64_t x;
            cin >> x;
            b[i][j] = (x == 1);
        }
    }

    vector<vector<int64_t>> d(m, vector<int64_t>(n, -1L));
    int64_t idx = 0;
    for (auto i : irange(0L, m)) {
        for (auto j : irange(0L, n)) {
            if (a[i][j] != b[i][j]) {
                d[i][j] = idx++;
            }
        }
    }

    vector<vector<int64_t>> adjs(idx);
    for (auto i : irange(0L, m)) {
        for (auto j : irange(0L, n)) {
            if (d[i][j] < 0 || a[i][j]) {
                continue;
            }

            const int di[4]{-1, 1, 0, 0};
            const int dj[4]{0, 0, -1, 1};
            for (auto k : irange(0L, 4L)) {
                auto ni = i + di[k];
                auto nj = j + dj[k];
                if (!(0 <= ni && ni < m) || !(0 <= nj && nj < n)) {
                    continue;
                }

                if (d[ni][nj] < 0) {
                    continue;
                }

                if (a[ni][nj] == a[i][j]) {
                    continue;
                }

                adjs[d[i][j]].push_back(d[ni][nj]);
            }
        }
    }

    int64_t mm = 0;
    vector<int64_t> match(idx, -1L);
    for (auto i : irange(0L, idx)) {
        if (match[i] < 0) {
            vector<bool> used(idx);
            if (dfs(adjs, match, used, i)) {
                ++mm;
            }
        }
    }

    cout << idx - mm << endl;
}