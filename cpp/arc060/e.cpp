#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> x(n);
    for (auto&& xx : x) {
        cin >> xx;
    }

    int64_t l, q;
    cin >> l >> q;
    vector<pair<int64_t, int64_t>> ab(q);
    for (auto&& t : ab) {
        cin >> t.first >> t.second;
        --t.first;
        --t.second;
        if (t.first > t.second) {
            swap(t.first, t.second);
        }
    }

    constexpr auto D = 20L;

    vector<vector<int64_t>> next(D, vector<int64_t>(n + 1, n));
    int64_t j = 1;
    for (auto i : irange(0L, n - 1)) {
        while (j + 1 < n && x[j + 1] - x[i] <= l) {
            ++j;
        }

        next[0][i] = j;
    }

    for (auto d : irange(1L, D)) {
        for (auto i : irange(0L, n - 1)) {
            next[d][i] = next[d - 1][next[d - 1][i]];
        }
    }

    for (const auto& t : ab) {
        int64_t ans = 0;
        int64_t i = t.first;

        for (auto d : irange(0L, D) | reversed) {
            if (next[d][i] <= t.second) {
                i = next[d][i];
                ans += (1L << d);
            }
        }

        if (i < t.second) {
            ++ans;
        }

        cout << ans << "\n";
    }
}