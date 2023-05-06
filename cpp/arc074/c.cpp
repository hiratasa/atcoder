#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h, w;
    cin >> h >> w;

    if (h % 3 == 0 || w % 3 == 0) {
        cout << 0 << endl;
        return 0;
    }

    int64_t ans = min(h, w);
    for (auto i : irange(1L, h)) {
        auto smax = max({(w + 1) / 2 * i, (h - i) * w});
        auto smin = min({w / 2 * i, (h - i) * w});
        ans = min(ans, smax - smin);
    }
    for (auto i : irange(1L, w)) {
        auto smax = max({(h + 1) / 2 * i, (w - i) * h});
        auto smin = min({h / 2 * i, (w - i) * h});
        ans = min(ans, smax - smin);
    }
    cout << ans << endl;
}