#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> c(13);
    for (auto i : irange(0L, n)) {
        int64_t d;
        cin >> d;
        ++c[d];
    }

    if (c[0] >= 1 || c[12] >= 2) {
        cout << 0 << endl;
        return 0;
    }

    vector<int64_t> h, h1;
    h1.push_back(0L);

    for (auto i : irange(1L, 13L)) {
        if (c[i] >= 3) {
            cout << 0 << endl;
            return 0;
        }

        if (c[i] == 2) {
            h1.push_back(i);
            h1.push_back(24 - i);
        }

        if (c[i] == 1) {
            h.push_back(i);
        }
    }

    int64_t ans = 0;
    for (auto bs0 : irange(0uL, 1uL << h.size())) {
        bitset<12> bs(bs0);
        auto h2 = h1;

        for (auto i : irange(0uL, h.size())) {
            if (bs[i]) {
                h2.push_back(h[i]);
            } else {
                h2.push_back(24 - h[i]);
            }
        }

        int64_t s = 24;
        for (auto i : irange(0uL, h2.size())) {
            for (auto j : irange(0uL, h2.size())) {
                if (i != j) {
                    auto d = abs(h2[i] - h2[j]);
                    s = min(s, min(d, 24 - d));
                }
            }
        }

        ans = max(ans, s);
    }

    cout << ans << endl;
}