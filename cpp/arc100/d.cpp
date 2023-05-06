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
    for (auto i : irange(0L, n)) {
        cin >> a[i];
    }

    vector<int64_t> s(n + 1);
    for (auto i : irange(0L, n)) {
        s[i + 1] = s[i] + a[i];
    }

    int64_t ans = numeric_limits<int64_t>::max();
    for (auto i : irange(1L, n - 2)) {
        auto sl = s[i + 1];
        auto sr = s[n] - sl;

        auto it = upper_bound(s.begin(), s.end(), sl / 2);
        auto ssl = *it;
        if ((sl - *(it - 1)) - *(it - 1) < ssl - (sl - ssl)) {
            ssl = *(it - 1);
        }

        auto it2 = upper_bound(s.begin(), s.end(), sl + sr / 2);
        auto ssr = *it2 - sl;
        if ((sr - (*(it2 - 1) - sl)) - (*(it2 - 1) - sl) < ssr - (sr - ssr)) {
            ssr = *(it2 - 1) - sl;
        }

        auto ma = max({ssl, sl - ssl, ssr, sr - ssr});
        auto mi = min({ssl, sl - ssl, ssr, sr - ssr});

        ans = min({ans, ma - mi});
    }

    cout << ans << endl;
}