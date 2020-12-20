#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(1L << n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    vector<array<int64_t, 2>> m(a.size(), {a.size(), a.size()});
    a.push_back(0);
    for (auto k : irange(0uL, 1uL << n)) {
        bitset<18> bs(k);

        m[k][0] = k;
        for (auto i : irange(0L, n)) {
            if (!bs[i]) {
                continue;
            }

            auto p = (k & ~(1uL << i));
            if (a[m[p][0]] > a[m[k][0]]) {
                m[k][1] = m[k][0];
                m[k][0] = m[p][0];
            } else if (m[p][0] != m[k][0] && a[m[p][0]] > a[m[k][1]]) {
                m[k][1] = m[p][0];
            }

            if (a[m[p][1]] > a[m[k][1]]) {
                m[k][1] = m[p][1];
            }
        }
    }

    int64_t prev = 0;
    for (auto i : irange(1uL, 1uL << n)) {
        int64_t ans = max(prev, a[m[i][0]] + a[m[i][1]]);
        prev = ans;
        cout << ans << "\n";
    }
}