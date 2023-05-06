#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    vector<bool> f(n);
    int64_t prev = -1;
    for (auto&& aa : a) {
        cin >> aa;

        if (aa == prev + 1) {
            f[prev - 1] = true;
        }

        prev = aa;
    }

    int64_t ans = 0;
    for (auto i : irange(0L, n)) {
        int64_t b;
        cin >> b;
        ans += b;
    }

    for (auto i : irange(0L, n - 1)) {
        int64_t c;
        cin >> c;
        if (f[i]) {
            ans += c;
        }
    }

    cout << ans << endl;
}