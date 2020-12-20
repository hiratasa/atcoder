#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n), b(n);
    for (auto&& aa : a) {
        cin >> aa;
    }
    for (auto&& bb : b) {
        cin >> bb;
    }

    int64_t ans = -1;
    for (auto k : irange(0uL, 1uL << n)) {
        bitset<18> bs(k);

        vector<pair<int64_t, int64_t>> e, o;
        for (auto i : irange(0L, n)) {
            if ((i + bs[i]) % 2 > 0) {
                o.emplace_back((bs[i] == 0 ? a[i] : b[i]), i);
            } else {
                e.emplace_back((bs[i] == 0 ? a[i] : b[i]), i);
            }
        }

        if (!(e.size() == o.size() || e.size() == o.size() + 1)) {
            continue;
        }

        sort(e.begin(), e.end());
        sort(o.begin(), o.end());

        int64_t prev = -1;
        bool ok = true;
        for (auto i : irange(0L, n)) {
            int64_t current = (i % 2 == 0 ? e[i / 2] : o[i / 2]).first;
            if (prev > current) {
                ok = false;
                break;
            }
            prev = current;
        }
        if (!ok) {
            continue;
        }

        vector<int64_t> c(n);
        for (auto i : irange(0uL, e.size())) {
            c[e[i].second] = 2 * i;
        }
        for (auto i : irange(0uL, o.size())) {
            c[o[i].second] = 2 * i + 1;
        }

        // bubble sort
        int64_t m = 0;
        for (auto i : irange(0L, n - 1)) {
            for (auto j : irange(1L, n - i)) {
                if (c[j - 1] > c[j]) {
                    swap(c[j - 1], c[j]);
                    ++m;
                }
            }
        }

        if (ans < 0 || ans > m) {
            ans = m;
        }
    }

    cout << ans << endl;
}