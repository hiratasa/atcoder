#include <bits/stdc++.h>

#include <atcoder/maxflow>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace atcoder;
using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector b(n, vector<bool>(m));
    vector<string> t(n);
    for (auto i : irange(0L, n)) {
        string s;
        cin >> s;
        for (auto j : irange(0L, m)) {
            b[i][j] = (s[j] == '.');
        }
        t[i] = s;
    }

    mf_graph<int64_t> g(n * m + 2);
    int64_t src = n * m, dst = n * m + 1;

    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, m)) {
            if (!b[i][j]) {
                continue;
            }

            bool is_even = (i + j) % 2;
            int64_t idx = i * m + j;

            if (is_even) {
                g.add_edge(src, idx, 1);
            } else {
                g.add_edge(idx, dst, 1);
            }

            if (!is_even) {
                continue;
            }

            int64_t dxs[] = {-1, 1, 0, 0};
            int64_t dys[] = {0, 0, -1, 1};
            for (auto k : irange(0L, 4L)) {
                auto dx = dxs[k];
                auto dy = dys[k];

                if (0 <= i + dx && i + dx < n && 0 <= j + dy && j + dy < m) {
                    if (!b[i + dx][j + dy]) {
                        continue;
                    }

                    int64_t idx2 = (i + dx) * m + j + dy;
                    g.add_edge(idx, idx2, 1);
                }
            }
        }
    }

    auto f = g.flow(src, dst);

    for (const auto& edge : g.edges()) {
        if (edge.from == src || edge.to == dst) {
            continue;
        }

        if (edge.flow == 0) {
            continue;
        }

        int64_t i0 = edge.from / m, j0 = edge.from % m;
        int64_t i1 = edge.to / m, j1 = edge.to % m;

        if (i0 == i1) {
            if (j0 > j1) {
                swap(j0, j1);
            }
            t[i0][j0] = '>';
            t[i0][j1] = '<';
        } else {
            if (i0 > i1) {
                swap(i0, i1);
            }
            t[i0][j0] = 'v';
            t[i1][j0] = '^';
        }
    }

    cout << f << endl;
    for (const auto& tt : t) {
        cout << tt << endl;
    }
}