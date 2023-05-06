#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s;
    cin >> s;

    for (auto&& c : s) {
        c -= '0';
    }

    vector<int64_t> t(2019);
    t[0] = 1;
    int64_t m = 0, d = 1;
    for (auto i : irange(0uL, s.size()) | reversed) {
        m += d * s[i];
        m %= 2019;
        d *= 10;
        d %= 2019;
        ++t[m];
    }

    int64_t ans = 0;
    for (auto i : irange(0L, 2019L)) {
        ans += t[i] * (t[i] - 1) / 2;
    }

    cout << ans << endl;
}