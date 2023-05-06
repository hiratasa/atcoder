#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t count(const vector<pair<int64_t, int64_t>>& ap, int64_t c) {
    int64_t l = 0;

    for (const auto& t : ap) {
        auto a = t.first;
        auto d = t.second;

        if (a > c) {
            continue;
        }

        // a + d * (i - 1) <= c
        auto m = (c - a) / d + 1;

        l += m;
    }

    return l;
}

int main() {
    int64_t k, n;
    cin >> k >> n;

    vector<pair<int64_t, int64_t>> ap(n);
    for (auto i : irange(0L, n)) {
        cin >> ap[i].first >> ap[i].second;
    }

    auto r = irange(1L, 1L << 40);
    auto c = *partition_point(r.begin(), r.end(),
                              [&](int64_t c) { return count(ap, c) < k; });
    --c;

    int64_t ans = 0;
    for (const auto& t : ap) {
        auto a = t.first;
        auto d = t.second;

        if (a > c) {
            continue;
        }

        // a + d * (i - 1) <= c
        auto m = (c - a) / d + 1;

        ans += m * a + m * (m - 1) / 2 * d;
    }

    ans += (k - count(ap, c)) * (c + 1);

    cout << ans << endl;
}