#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<std::tuple<int64_t, int64_t, int64_t>> xyz(n);
    for (auto&& t : xyz) {
        auto& [x, y, z] = t;
        cin >> x >> y >> z;
    }

    auto cost = [&](int64_t i, int64_t j) {
        auto [x0, y0, z0] = xyz[i];
        auto [x1, y1, z1] = xyz[j];
        return abs(x0 - x1) + abs(y0 - y1) + max(0L, z1 - z0);
    };

    vector costs(1L << (n - 1), vector(n, 1L << 30));
    costs[0][0] = 0;
    for (auto u : irange(1uL, 1uL << (n - 1))) {
        bitset<17> bs(u);

        for (auto current : irange(1L, n)) {
            if (!bs[current - 1]) {
                continue;
            }

            for (auto prev : irange(0L, n)) {
                if (prev > 0 && !bs[prev - 1]) {
                    continue;
                }
                if (prev == current) {
                    continue;
                }

                costs[u][current] =
                        min(costs[u][current],
                            costs[u & ~(1uL << (current - 1))][prev] +
                                    cost(prev, current));
            }
        }
    }

    int64_t ans = numeric_limits<int64_t>::max();
    for (auto i : irange(1L, n)) {
        ans = min(ans, costs[(1uL << (n - 1)) - 1][i] + cost(i, 0));
    }

    cout << ans << endl;
}