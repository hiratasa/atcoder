#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t solve(const vector<pair<int64_t, int64_t>>& vw, int64_t v, int64_t l) {
    int64_t n = vw.size();

    static vector<unordered_map<int64_t, int64_t>> memo(n);

    if (l < 0) {
        return numeric_limits<int64_t>::min();
    }

    if (v == 0) {
        if (vw[0].second <= l) {
            return vw[0].first;
        } else {
            return 0;
        }
    }

    if (memo[v].count(l)) {
        return memo[v][l];
    }

    return memo[v][l] =
                   max(solve(vw, (v - 1) / 2, l),
                       solve(vw, (v - 1) / 2, l - vw[v].second) + vw[v].first);
}

int main() {
    int64_t n;
    cin >> n;

    vector vw(n, make_pair(0L, 0L));
    for (auto&& t : vw) {
        cin >> t.first >> t.second;
    }

    int64_t q;
    cin >> q;
    for (auto _ : irange(0L, q)) {
        int64_t v, l;
        cin >> v >> l;
        --v;

        cout << solve(vw, v, l) << "\n";
    }
}