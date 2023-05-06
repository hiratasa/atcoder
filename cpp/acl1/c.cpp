#include <bits/stdc++.h>

#include <atcoder/mincostflow>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;
using namespace atcoder;

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector mat(n, vector<bool>(m));
    vector<pair<int64_t, int64_t>> idxs;
    for (auto i : irange(0L, n)) {
        string s;
        cin >> s;
        for (auto j : irange(0L, m)) {
            if (s[j] == '#') {
                continue;
            }

            mat[i][j] = true;
            if (s[j] == 'o') {
                idxs.emplace_back(i, j);
            }
        }
    }

    mcf_graph<int64_t, int64_t> g(n * m + 2);
    int64_t src = n * m, dst = n * m + 1;

    auto IDX = [&](int64_t i, int64_t j) { return i * m + j; };

    for (auto [i, j] : idxs) {
        g.add_edge(src, IDX(i, j), 1, i + j);
    }

    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, m)) {
            if (mat[i][j]) {
                g.add_edge(IDX(i, j), dst, 1, n + m - i - j);
            }
        }
    }

    for (auto i : irange(0L, n - 1)) {
        for (auto j : irange(0L, m)) {
            if (mat[i][j] && mat[i + 1][j]) {
                g.add_edge(IDX(i, j), IDX(i + 1, j), idxs.size(), 0);
            }
        }
    }

    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, m - 1)) {
            if (mat[i][j] && mat[i][j + 1]) {
                g.add_edge(IDX(i, j), IDX(i, j + 1), idxs.size(), 0);
            }
        }
    }

    auto [f, c] = g.flow(src, dst, idxs.size());

    assert(f == idxs.size());

    // for (const auto& edge : g.edges()) {
    //     cerr << edge.from << "=>" << edge.to << ", cap=" << edge.cap
    //          << ", cost=" << edge.cost << ", flow=" << edge.flow << endl;
    // }

    cout << -c + (n + m) * idxs.size() << endl;
}