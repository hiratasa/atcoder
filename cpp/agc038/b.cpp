#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n, k;
    cin >> n >> k;

    vector<int64_t> p(n);
    for (auto&& pp : p) {
        cin >> pp;
    }

    vector<bool> sorted(n, false);
    int64_t cur = 0;
    int64_t prev = -1;
    for (auto i : irange(0L, n)) {
        if (p[i] < prev) {
            cur = i;
        }
        if (i - cur + 1 >= k) {
            sorted[i - k + 1] = true;
        }
        prev = p[i];
    }

    multiset<int64_t> s;
    for (auto i : irange(0L, k)) {
        s.insert(p[i]);
    }

    bool has_sorted = false;
    int64_t ans = 1;
    vector<int64_t> pos;
    if (sorted[0]) {
        has_sorted = true;
    }
    for (int64_t i = 1; i <= n - k; ++i) {
        if (!(*s.begin() == p[i - 1] && *s.rbegin() <= p[i + k - 1])) {
            if (!sorted[i]) {
                ++ans;
            } else {
                if (!has_sorted) {
                    ++ans;
                }
                has_sorted = true;
            }
        }

        s.erase(s.find(p[i - 1]));
        s.insert(p[i + k - 1]);
    }

    cout << ans << endl;
}