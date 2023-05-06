#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

pair<int64_t, int64_t> dfs(const vector<vector<int64_t>>& children, int64_t v,
                           int64_t k) {
    int64_t d = 0;
    int64_t r = 0;

    for (auto u : children[v]) {
        auto [dd, rr] = dfs(children, u, k);
        ++dd;

        if (v > 0 && dd >= k) {
            ++r;
            dd = 0;
        }

        d = max(d, dd);
        r += rr;
    }

    return make_pair(d, r);
}

int main() {
    int64_t n, k;
    cin >> n >> k;

    int64_t a0;
    cin >> a0;
    --a0;

    vector<vector<int64_t>> children(n);
    for (auto i : irange(1L, n)) {
        int64_t a;
        cin >> a;
        --a;

        children[a].push_back(i);
    }

    auto [_, ans] = dfs(children, 0L, k);
    ans += (a0 == 0 ? 0 : 1);

    cout << ans << endl;
}