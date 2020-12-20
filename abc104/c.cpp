#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t d, g;
    cin >> d >> g;
    g /= 100;

    vector<array<int64_t, 2>> pc(d);
    for (auto&& t : pc) {
        cin >> t[0] >> t[1];
        t[1] /= 100;
    }

    int64_t ans = numeric_limits<int64_t>::max();
    for (auto s : irange(0uL, 1uL << d)) {
        bitset<10> bs(s);

        int64_t points = 0;
        int64_t k = 0;
        for (auto i : irange(0L, d)) {
            if (bs[i]) {
                points += pc[i][0] * (i + 1) + pc[i][1];
                k += pc[i][0];
            }
        }

        for (auto i : irange(0L, d) | reversed) {
            if (!bs[i]) {
                auto m = min(pc[i][0] - 1, (max(g - points, 0L) + i) / (i + 1));
                points += m * (i + 1);
                k += m;
            }
        }

        if (points >= g) {
            ans = min(ans, k);
        }
    }

    cout << ans << endl;
}