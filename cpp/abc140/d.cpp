#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, k;
    cin >> n >> k;

    string s;
    cin >> s;

    int64_t b = 0;
    char prev = -1;
    for (auto c : s) {
        if (prev != c) {
            ++b;
            prev = c;
        }
    }

    int64_t ans = n - b;
    if (b >= 3) {
        int64_t a = min((b - 1) / 2, k);
        b -= 2 * a;
        k -= a;
        ans += 2 * a;
    }

    if (b == 2 && k >= 1) {
        --b;
        --k;
        ans += 1;
    }

    cout << ans << endl;
}