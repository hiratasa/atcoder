#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s;
    cin >> s;

    constexpr int64_t K = 'z' - 'a' + 1;
    vector<int64_t> a(K, 0L), m(K, 1L << 20), mi(K, 0L);

    for (auto i : irange(0L, static_cast<int64_t>(s.size()))) {
        auto c = s[i];

        if (2 * (a[c - 'a'] + 1) - i > m[c - 'a']) {
            cout << mi[c - 'a'] + 1 << " " << i + 1 << endl;
            return 0;
        }

        for (auto c1 : irange(0L, K)) {
            if (2 * a[c1] - (i - 1) < m[c1]) {
                m[c1] = 2 * a[c1] - (i - 1);
                mi[c1] = i;
            }
        }

        ++a[c - 'a'];
    }

    cout << "-1 -1" << endl;
}