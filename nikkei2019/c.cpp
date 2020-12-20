#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n), b(n);
    vector<int64_t> s(n);
    int64_t sb = 0;
    for (auto i : irange(0L, n)) {
        cin >> a[i] >> b[i];
        s[i] = a[i] + b[i];
        sb += b[i];
    }

    sort(s.rbegin(), s.rend());
    int64_t ans = -sb;
    for (int64_t i = 0; i < n; i += 2) {
        ans += s[i];
    }

    cout << ans << endl;
}