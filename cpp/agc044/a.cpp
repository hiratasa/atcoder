#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t t;
    cin >> t;

    for (auto _ : irange(0L, t)) {
        int64_t n, d;
        array<int64_t, 3> c;
        cin >> n >> c[0] >> c[1] >> c[2] >> d;

        unordered_map<int64_t, int64_t> costs;
        costs[n] = 0;

        priority_queue<pair<int64_t, int64_t>, vector<pair<int64_t, int64_t>>,
                       std::greater<>>
                q;
        q.emplace(0L, n);

        while (!q.empty()) {
            auto [cost, m] = q.top();
            q.pop();

            if (m == 0) {
                cout << cost << endl;
                break;
            }

            array e{2L, 3L, 5L};
            for (auto i : irange(0L, 3L)) {
                for (auto k : irange(max(1L, m - e[i] + 1), m + e[i])) {
                    if (k % e[i] == 0) {
                        auto next = k / e[i];
                        auto new_cost = cost + abs(k - m) * d + c[i];
                        if (costs.count(next) == 0 || costs[next] > new_cost) {
                            costs[next] = new_cost;
                            q.emplace(new_cost, next);
                        }
                    }
                }
            }

            if (m < ((1L << 60) - cost) / d) {
                auto new_cost = cost + m * d;
                if (costs.count(0) == 0 || costs[0] > new_cost) {
                    costs[0] = new_cost;
                    q.emplace(new_cost, 0L);
                }
            }
        }
    }
}