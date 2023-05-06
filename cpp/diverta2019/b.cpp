#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t r, g, b, n;
    cin >> r >> g >> b >> n;

    int64_t ans = 0;
    for (int64_t i = 0; i * r <= n; ++i) {
        int64_t m = n - i * r;
        for (int64_t j = 0; j * g <= m; ++j) {
            if ((n - i * r - j * g) % b == 0) {
                ++ans;
            }
        }
    }
      

    cout << ans << endl;
}