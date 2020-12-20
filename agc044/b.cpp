#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector p(n * n, 0L);
    for (auto i : irange(0L, n * n)) {
        cin >> p[i];
        --p[i];
    }

    vector costs(n * n, 0L);
    for (auto i : irange(0L, n * n)) {
        auto r = i / n;
        auto c = i % n;

        costs[i] = min({r, c, n - r - 1, n - c - 1});
    }

    array dxs{-1L, 1L, 0L, 0L};
    array dys{0L, 0L, -1L, 1L};

    vector absent(n * n, false);
    int64_t ans = 0;
    for (auto pp : p) {
        // cerr << pp << ":" << costs[pp] << endl;
        ans += costs[pp];

        absent[pp] = true;

        vector st(1L, pp);
        while (!st.empty()) {
            auto v = st.back();
            st.pop_back();

            auto r = v / n;
            auto c = v % n;
            auto cost = costs[v] + (absent[v] ? 0 : 1);
            for (auto i : irange(0L, 4L)) {
                auto nr = r + dxs[i];
                auto nc = c + dys[i];
                if (nr < 0 || nr >= n || nc < 0 || nc >= n) {
                    continue;
                }

                auto u = nr * n + nc;

                // cerr << v << "->" << u << ":" << costs[u] << "=>" << cost
                    //  << endl;
                if (cost < costs[u]) {
                    costs[u] = cost;
                    st.push_back(u);
                }
            }
        }
    }

    cout << ans << endl;
}