#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

constexpr auto INVALID = 1L << 30;

bool calc_range(const vector<vector<int64_t>>& adjs, const vector<int64_t>& p,
                vector<pair<int64_t, int64_t>>& range, int64_t v,
                int64_t prev) {
    auto& r = range[v];
    if (p[v] != -1) {
        r.first = r.second = p[v];
    }

    for (auto u : adjs[v]) {
        if (u == prev) {
            continue;
        }

        if (!calc_range(adjs, p, range, u, v)) {
            return false;
        }

        const auto& r2 = range[u];
        if (r2.first != INVALID) {
            if (r.first == INVALID) {
                r.first = r2.first - 1;
                r.second = r2.second + 1;
            } else if ((r.first - (r2.first - 1)) % 2 != 0) {
                return false;
            } else {
                r.first = max(r.first, r2.first - 1);
                r.second = min(r.second, r2.second + 1);
            }
        }
    }

    return r.first <= r.second;
}

void assign_value(const vector<vector<int64_t>>& adjs,
                  const vector<pair<int64_t, int64_t>>& range,
                  vector<int64_t>& values, int64_t v, int64_t prev) {
    for (auto u : adjs[v]) {
        if (u == prev) {
            continue;
        }

        values[u] = values[v] - 1;
        if (range[u].first != INVALID && values[u] < range[u].first) {
            values[u] = values[v] + 1;
        }
        assert(range[u].second == INVALID || values[u] <= range[u].second);
        assign_value(adjs, range, values, u, v);
    }
}

int main() {
    int64_t n;
    cin >> n;

    vector<vector<int64_t>> adjs(n);
    for (auto _ : irange(0L, n - 1)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;
        adjs[a].push_back(b);
        adjs[b].push_back(a);
    }

    int64_t k;
    cin >> k;

    vector<int64_t> p(n, -1);
    for (auto _ : irange(0L, k)) {
        int64_t v;
        cin >> v;
        --v;
        cin >> p[v];
    }

    vector<pair<int64_t, int64_t>> range(n, make_pair(INVALID, INVALID));
    if (!calc_range(adjs, p, range, 0, -1)) {
        cout << "No" << endl;
        return 0;
    }

    vector<int64_t> ans(n);
    ans[0] = range[0].first;
    assign_value(adjs, range, ans, 0, -1);

    cout << "Yes" << endl;
    for (auto a : ans) {
        cout << a << "\n";
    }
}