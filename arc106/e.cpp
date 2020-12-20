#include <bits/stdc++.h>

#include <atcoder/maxflow>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

using namespace atcoder;

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector a(n, 0L);
    for (auto&& aa : a) {
        cin >> aa;

        if (aa >= k) {
        }
    }

    auto r = irange(1L, 1000000L);
    auto ans = *partition_point(r.begin(), r.end(), [&](int64_t d) {
        mf_graph<int64_t> g(d + 2 + n);

        for (auto i : irange(1L, d + 1)) {
            g.add_edge(0L, i, 1);

            for (auto j : irange(0L, n)) {
                if ((i - 1) / a[j] % 2 == 0) {
                    g.add_edge(i, d + 1 + j, 1);
                }
            }
        }

        for (auto i : irange(0L, n)) {
            for (auto _ : irange(0L, k)) {
                g.add_edge(d + 1 + i, d + n + 1, 1);
            }
        }

        auto f = g.flow(0, d + n + 1, k * n);

        return f < k * n;
    });

    cout << ans << endl;
}