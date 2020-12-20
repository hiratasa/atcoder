#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, x;
    cin >> n >> x;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    int64_t ans = 1L << 50;
    for (auto i : irange(0L, n)) {
        int64_t s = accumulate(a.begin(), a.end(), 0L);
        ans = min(s + i * x, ans);

        auto aa = a[n - 1];
        for (auto j : irange(1L, n) | reversed) {
            a[j] = min(a[j], a[j - 1]);
        }
        a[0] = min(a[0], aa);
    }

    cout << ans << endl;
}