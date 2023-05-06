#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<pair<string, int64_t>> st(n);
    for (auto&& v : st) {
        cin >> v.first >> v.second;
    }

    string x;
    cin >> x;

    int64_t ans = -1;
    for (auto i : irange(0L, n)) {
        if (ans >= 0) {
            ans += st[i].second;
        } else if (x == st[i].first) {
            ans = 0;
        }
    }

    cout << ans << endl;
}