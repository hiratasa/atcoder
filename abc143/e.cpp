#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n, m, l;
    cin >> n >> m >> l;

    vector<vector<pair<int64_t, int64_t>>> links(n);
    for (auto _ : irange(0L, m)) {
        int64_t a, b, c;
        cin >> a >> b >> c;
        if (c > l) {
            continue;
        }

        --a;
        --b;
        links[a].emplace_back(b, c);
        links[b].emplace_back(a, c);
    }

    constexpr auto kInfinity = numeric_limits<int64_t>::max() / 4L;

    vector<vector<int64_t>> dist(n, vector<int64_t>(n, kInfinity));
    for (auto i : irange(0L, n)) {
        dist[i][i] = 0;

        for (auto link : links[i]) {
            dist[i][link.first] = link.second;
        }
    }

    for (auto k : irange(0L, n)) {
        for (auto i : irange(0L, n)) {
            for (auto j : irange(0L, n)) {
                if (dist[i][j] > dist[i][k] + dist[k][j]) {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    vector<vector<int64_t>> nums(n, vector<int64_t>(n, kInfinity));
    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, n)) {
            if (i == j) {
                nums[i][j] = 0;
            } else if (dist[i][j] <= l) {
                nums[i][j] = 1;
            }
        }
    }

    for (auto k : irange(0L, n)) {
        for (auto i : irange(0L, n)) {
            for (auto j : irange(0L, n)) {
                if (nums[i][j] > nums[i][k] + nums[k][j]) {
                    nums[i][j] = nums[i][k] + nums[k][j];
                }
            }
        }
    }

    int64_t q;
    cin >> q;
    for (auto _ : irange(0L, q)) {
        int64_t s, t;
        cin >> s >> t;
        --s;
        --t;

        if (nums[s][t] == kInfinity) {
            cout << -1 << "\n";
        } else {
            assert(s != t);
            cout << nums[s][t] - 1 << "\n";
        }
    }
}