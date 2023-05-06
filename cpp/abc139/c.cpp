#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    int64_t prev = -1;
    int64_t current = 0, ans = 0;
    for (auto _ : irange(0L, n)) {
        int64_t h;
        cin >> h;
        if (h > prev) {
            current = 0;
        } else {
            ++current;
            ans = max(ans, current);
        }
        prev = h;
    }

    cout << ans << endl;
}