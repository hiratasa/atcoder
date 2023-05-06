#include <bits/stdc++.h>

#include <atcoder/mincostflow>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace atcoder;
using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector a(n, vector(n, 0L));
    for (auto&& aa : a) {
        for (auto&& aaa : aa) {
            cin >> aaa;
        }
    }

    constexpr auto M = 1000000000L;

    mcf_graph<int64_t, int64_t> g(2 * n + 2);

    int64_t src = 2 * n;
    int64_t dst = 2 * n + 1;

    g.add_edge(src, dst, n * k, M);

    for (auto i : irange(0L, n)) {
        g.add_edge(src, i, k, 0);
        g.add_edge(n + i, dst, k, 0);
    }

    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, n)) {
            g.add_edge(i, n + j, 1, M - a[i][j]);
        }
    }

    auto [f, c] = g.flow(src, dst, n * k);

    int64_t ans = M * f - c;

    cout << ans << endl;

    vector selected(n, vector(n, false));
    for (const auto& edge : g.edges()) {
        if (edge.flow > 0 && edge.from < n && n <= edge.to && edge.to < 2 * n) {
            selected[edge.from][edge.to - n] = true;
        }
    }

    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, n)) {
            cout << (selected[i][j] ? 'X' : '.');
        }
        cout << endl;
    }
}