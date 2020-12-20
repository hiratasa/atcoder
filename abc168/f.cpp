#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

void dfs(const vector<vector<bool>>& w1, const vector<vector<bool>>& w2,
         vector<vector<bool>>& visited, int64_t x0, int64_t y0) {
    vector<pair<int64_t, int64_t>> st;

    st.emplace_back(x0, y0);
    while (!st.empty()) {
        auto [x, y] = st.back();
        st.pop_back();
        visited[x][y] = true;

        if (!w1[x][y] && !visited[x][y - 1]) {
            st.emplace_back(x, y - 1);
        }
        if (!w1[x][y + 1] && !visited[x][y + 1]) {
            st.emplace_back(x, y + 1);
        }
        if (!w2[x][y] && !visited[x - 1][y]) {
            st.emplace_back(x - 1, y);
        }
        if (!w2[x + 1][y] && !visited[x + 1][y]) {
            st.emplace_back(x + 1, y);
        }
    }
}

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<int64_t> xs, ys;
    xs.push_back(-(1L << 30));
    xs.push_back(0);
    xs.push_back(1L << 30);
    ys.push_back(-(1L << 30));
    ys.push_back(0);
    ys.push_back(1L << 30);

    vector<int64_t> a(n), b(n), c(n), d(m), e(m), f(m);
    for (auto i : irange(0L, n)) {
        cin >> a[i] >> b[i] >> c[i];
        xs.push_back(a[i]);
        xs.push_back(b[i]);
        ys.push_back(c[i]);
    }
    for (auto i : irange(0L, m)) {
        cin >> d[i] >> e[i] >> f[i];
        xs.push_back(d[i]);
        ys.push_back(e[i]);
        ys.push_back(f[i]);
    }

    sort(xs.begin(), xs.end());
    xs.erase(unique(xs.begin(), xs.end()), xs.end());
    sort(ys.begin(), ys.end());
    ys.erase(unique(ys.begin(), ys.end()), ys.end());

    unordered_map<int64_t, int64_t> x_idx, y_idx;
    for (auto x : xs) {
        x_idx[x] = x_idx.size();
    }
    for (auto y : ys) {
        y_idx[y] = y_idx.size();
    }

    vector<vector<bool>> w1(x_idx.size(), vector(y_idx.size(), false));
    for (auto i : irange(0L, n)) {
        for (auto x : irange(x_idx[a[i]], x_idx[b[i]])) {
            w1[x][y_idx[c[i]]] = true;
        }
    }
    vector<vector<bool>> w2(x_idx.size(), vector(y_idx.size(), false));
    for (auto i : irange(0L, m)) {
        for (auto y : irange(y_idx[e[i]], y_idx[f[i]])) {
            w2[x_idx[d[i]]][y] = true;
        }
    }

    for (auto x : irange(0uL, x_idx.size())) {
        w1[x][0] = true;
        w1[x][y_idx.size() - 1] = true;
    }

    for (auto y : irange(0uL, y_idx.size())) {
        w2[0][y] = true;
        w2[x_idx.size() - 1][y] = true;
    }

    vector<vector<bool>> visited(x_idx.size() - 1,
                                 vector(y_idx.size() - 1, false));
    dfs(w1, w2, visited, x_idx[0], y_idx[0]);

    for (auto x : irange(0uL, x_idx.size() - 1)) {
        if (visited[x][0] || visited[x][y_idx.size() - 2]) {
            cout << "INF" << endl;
            return 0;
        }
    }

    for (auto y : irange(0uL, y_idx.size() - 1)) {
        if (visited[0][y] || visited[x_idx.size() - 2][y]) {
            cout << "INF" << endl;
            return 0;
        }
    }

    int64_t ans = 0;
    for (auto x : irange(1uL, x_idx.size() - 2)) {
        for (auto y : irange(1uL, y_idx.size() - 2)) {
            if (visited[x][y]) {
                ans += (xs[x + 1] - xs[x]) * (ys[y + 1] - ys[y]);
            }
        }
    }

    cout << ans << endl;
}