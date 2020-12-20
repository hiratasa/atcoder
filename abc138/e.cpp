#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    string s, t;
    cin >> s >> t;

    constexpr int64_t K = 'z' - 'a' + 1;
    vector<vector<int64_t>> a(s.size(), vector<int64_t>(K, -1));
    vector<int64_t> b(K, -1);
    for (auto i : irange(0uL, (s.size())) | reversed) {
        for (auto j : irange(0L, K)) {
            if (b[j] == -1) {
                continue;
            }
            a[i][j] = b[j] - i;
        }
        b[s[i] - 'a'] = i;
    }

    for (auto i : irange(0uL, (s.size())) | reversed) {
        for (auto j : irange(0L, K)) {
            if (a[i][j] == -1 && b[j] != -1) {
                a[i][j] = b[j] - i + s.size();
            }
        }
    }

    int64_t ans = b[t[0] - 'a'];
    if (ans == -1) {
        cout << -1 << endl;
        return 0;
    }
    for (auto i : irange(1uL, t.size())) {
        auto tmp = a[ans % s.size()][t[i] - 'a'];
        if (tmp == -1) {
            cout << -1 << endl;
            return 0;
        }
        ans += tmp;
    }
    ++ans;

    cout << ans << endl;
}