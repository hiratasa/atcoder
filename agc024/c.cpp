#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    if (a[0] != 0) {
        cout << -1 << endl;
        return 0;
    }

    int64_t ans = 0;
    for (auto i : irange(1L, n)) {
        if (a[i] > a[i - 1] + 1) {
            cout << -1 << endl;
            return 0;
        }

        if (a[i] != a[i - 1] + 1) {
            ans += a[i - 1];
        }
    }

    ans += a[n - 1];

    cout << ans << endl;
}