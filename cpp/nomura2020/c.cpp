#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector a(n + 1, 0L);
    for (auto&& aa : a) {
        cin >> aa;
    }

    if (n == 0) {
        if (a[0] == 1) {
            cout << 1 << endl;
        } else {
            cout << -1 << endl;
        }
        return 0;
    }

    if (a[0] != 0) {
        cout << -1 << endl;
        return 0;
    }

    vector s(n + 1, a[n]);
    for (auto i : irange(0L, n) | reversed) {
        s[i] = s[i + 1] + a[i];
    }

    int64_t p = 1;
    int64_t ans = 1;
    for (auto i : irange(1L, n + 1)) {
        p = min(2 * p, s[i]);
        if (p < a[i]) {
            cout << -1 << endl;
            return 0;
        }
        ans += p;
        // cerr << p << " " << s[i] << " " << p - a[i] << "\n";
        p -= a[i];
    }

    cout << ans << endl;
}