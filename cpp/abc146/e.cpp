#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n, k;
    cin >> n >> k;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    unordered_map<int64_t, int64_t> m;
    int64_t ans = 0;
    vector<int64_t> s(n + 1);
    ++m[0];
    for (auto i : irange(0L, n)) {
        if (i - k + 1 >= 0) {
            --m[(s[i - k + 1] + (i - k + 1) * (k - 1)) % k];
        }

        s[i + 1] = s[i] + a[i];
        ans += m[(s[i + 1] + (i + 1) * (k - 1)) % k];
        ++m[(s[i + 1] + (i + 1) * (k - 1)) % k];
    }

    cout << ans << endl;
}